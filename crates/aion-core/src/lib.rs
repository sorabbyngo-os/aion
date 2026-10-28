//! AION core: intent -> plan -> authorize -> mediate -> result.
//! Models propose; policy decides; the mediator is the only thing that acts.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub id: String,
    pub source: String,
    pub text: String,
}

/// Typed actions only. Model output is untrusted and must parse into one of these.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Action {
    Respond { text: String },
    // Future: typed ObjectSpace actions go here.
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Plan {
    pub steps: Vec<Action>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "decision", content = "reason", rename_all = "snake_case")]
pub enum Decision {
    Allow,
    Confirm(String),
    Deny(String),
}

pub trait Backend: Send + Sync {
    fn name(&self) -> &str;
    fn plan(&self, intent: &Intent) -> Result<Plan, String>;
}

pub trait Policy: Send + Sync {
    fn authorize(&self, intent: &Intent, action: &Action) -> Decision;
}

pub trait Mediator: Send + Sync {
    fn execute(&self, action: &Action) -> Result<String, String>;
}

#[derive(Default)]
pub struct Registry {
    backends: BTreeMap<String, Box<dyn Backend>>,
    default: Option<String>,
}

impl Registry {
    pub fn register(&mut self, b: Box<dyn Backend>) {
        let name = b.name().to_string();
        if self.default.is_none() {
            self.default = Some(name.clone());
        }
        self.backends.insert(name, b);
    }

    pub fn select(&self, preferred: Option<&str>) -> Option<&dyn Backend> {
        let key = preferred.map(str::to_string).or_else(|| self.default.clone())?;
        self.backends.get(&key).map(|b| b.as_ref())
    }
}

#[derive(Debug, Serialize)]
pub struct StepResult {
    pub action: Action,
    pub decision: Decision,
    pub output: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Outcome {
    pub intent_id: String,
    pub backend: String,
    pub steps: Vec<StepResult>,
}

pub struct Pipeline {
    pub registry: Registry,
    pub policy: Box<dyn Policy>,
    pub mediator: Box<dyn Mediator>,
}

impl Pipeline {
    pub fn handle(&self, intent: &Intent, backend: Option<&str>) -> Result<Outcome, String> {
        let b = self
            .registry
            .select(backend)
            .ok_or("no backend available")?;
        let plan = b.plan(intent)?;
        let mut steps = Vec::new();
        for action in plan.steps {
            let decision = self.policy.authorize(intent, &action);
            let (output, error) = match &decision {
                Decision::Allow => match self.mediator.execute(&action) {
                    Ok(o) => (Some(o), None),
                    Err(e) => (None, Some(e)),
                },
                Decision::Confirm(_) => (None, Some("confirmation required".to_string())),
                Decision::Deny(reason) => (None, Some(reason.clone())),
            };
            steps.push(StepResult { action, decision, output, error });
        }
        Ok(Outcome {
            intent_id: intent.id.clone(),
            backend: b.name().to_string(),
            steps,
        })
    }
}

pub mod defaults {
    use super::*;

    pub struct EchoBackend;
    impl Backend for EchoBackend {
        fn name(&self) -> &str {
            "echo"
        }
        fn plan(&self, intent: &Intent) -> Result<Plan, String> {
            Ok(Plan {
                steps: vec![Action::Respond { text: format!("echo: {}", intent.text) }],
            })
        }
    }

    /// Default-deny: any action not explicitly listed here is refused.
    pub struct DefaultPolicy;
    impl Policy for DefaultPolicy {
        fn authorize(&self, _intent: &Intent, action: &Action) -> Decision {
            match action {
                Action::Respond { .. } => Decision::Allow,
            }
        }
    }

    pub struct LogMediator;
    impl Mediator for LogMediator {
        fn execute(&self, action: &Action) -> Result<String, String> {
            match action {
                Action::Respond { text } => Ok(text.clone()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::defaults::*;
    use super::*;

    fn intent(text: &str) -> Intent {
        Intent { id: "1".into(), source: "test".into(), text: text.into() }
    }

    fn pipeline(policy: Box<dyn Policy>) -> Pipeline {
        let mut registry = Registry::default();
        registry.register(Box::new(EchoBackend));
        Pipeline { registry, policy, mediator: Box::new(LogMediator) }
    }

    #[test]
    fn echo_is_allowed_and_executed() {
        let out = pipeline(Box::new(DefaultPolicy)).handle(&intent("hi"), None).unwrap();
        assert_eq!(out.steps.len(), 1);
        assert_eq!(out.steps[0].decision, Decision::Allow);
        assert_eq!(out.steps[0].output.as_deref(), Some("echo: hi"));
    }

    #[test]
    fn denied_actions_never_reach_the_mediator() {
        struct DenyAll;
        impl Policy for DenyAll {
            fn authorize(&self, _: &Intent, _: &Action) -> Decision {
                Decision::Deny("nope".into())
            }
        }
        let out = pipeline(Box::new(DenyAll)).handle(&intent("hi"), None).unwrap();
        assert_eq!(out.steps[0].output, None);
        assert_eq!(out.steps[0].error.as_deref(), Some("nope"));
    }

    #[test]
    fn empty_registry_is_an_error() {
        let p = Pipeline {
            registry: Registry::default(),
            policy: Box::new(DefaultPolicy),
            mediator: Box::new(LogMediator),
        };
        assert!(p.handle(&intent("hi"), None).is_err());
    }
}
