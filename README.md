# AION

**AI runtime and control plane for Sorabbyngo OS**

> Models reason.  
> AION decides, routes, and mediates.  
> The system never hands the keys to a vendor or a weight file.

AION is not an AI model.  
It is the layer that turns **intent into safe, observable action** —  
whether that intent comes from Chameha, Vanta’s UI, or another node on Byngonet.

---

## Vision

Intelligence in an OS should feel native: always available, never mandatory, never opaque.

AION exists so that:

- Local models, containerized servers, and cloud APIs look the **same** from the desktop’s point of view  
- Context can move when the user switches model or machine  
- Every effect on the system passes through **one mediated door**  
- The compositor and the user stay in control when models are wrong, slow, or offline  

```text
Intent (Chameha / Vanta / remote)
        ↓
     AION
        ├── select backend (local | API | remote AION)
        ├── assemble context (via memory service)
        ├── plan / tool calls
        ├── authorize & mediate
        └── execute through ObjectSpace / typed actions
        ↓
   Observable result → Vanta / speech
Without AION, “AI OS” collapses into either a chatbot or root-level scripts.
With AION, intelligence is infrastructure.
Responsibilities
AION owns:
Receiving structured intent
Planning and decomposing tasks
Choosing backends by policy, capability, and availability
Assembling working context for the chosen backend
Validating tool calls and permissions
Mediating execution (ObjectSpace / action API)
Coordinating multi-device handoff (with Byngonet)
Emitting results suitable for UI and Chameha
AION does not own:
Concern
Owner
Voice personality & STT/TTS
Chameha
Pixels, seats, tiling
Vanta
Long-term memory store
Context / Memory service
Mesh transport
Byngonet
Process supervision of the whole OS
systemd + Sentinel
A particular model vendor
User / packager
Stay a control plane. Refuse to become systemd-with-LLM.
Backends
All intelligence providers register uniformly:
Type
Examples
Role
Local
System or user-installed weights
Default offline path
Containerized
Ollama, vLLM, …
HTTP/API backends under policy
Cloud
User-supplied API keys
Opt-in; filter sensitive context
Remote node
Another machine’s AION via Byngonet
Continuation / offload
Registration is capability-based.
Selection is policy-based.
No backend receives raw root, raw device control, or silent filesystem write.
Context and continuity
Working context     → assembled per request for the active backend
Project / chat memory → owned by a memory service; AION reads/writes
Context switch      → AION decides *when*; memory service performs *how*
Model switch        → new backend receives portable task/state summary, not a vendor lock-in blob
A developer can start with a local model, later attach a cloud subscription, and continue the same project state — because state lives in the system, not inside one provider’s session.
Safety model
Model output
    → parse structured tools / plan
    → permission & policy check
    → user confirm when required
    → execute via ObjectSpace / typed actions
    → log · undo surface where possible
Least privilege.
One door.
Direct path still works if AION is degraded.
Optional CML-style filters may sit on outbound cloud calls (privacy, regional policy). They do not replace mediation.
Technical direction
Area
Approach
Language
Rust preferred for the runtime core
IPC
Stable contracts with Vanta (vanta-intent) and ObjectSpace
Backends
HTTP/gRPC/OpenAI-compatible where practical; local process adapters
Isolation
systemd units, cgroups, containers, SELinux types (Fedora lineage)
Deployment
Service on the immutable base; not in-kernel
Offline
Local backend path is first-class
Non-goals: multi-agent product platform, replacing the compositor, mandatory cloud, hard-coded single vendor.
Relation to older research
Earlier idea
Current
Planning inside Chameha
AION
Byngox as multi-agent cognitive core
Superseded — models are pluggable backends
Intelligence as a branded closed model
Rejected — user-chosen models
Status
Design and early implementation phase.
Near-term goals:
Intent → plan → mediate → act pipeline
Uniform backend registration
Clean boundaries with Chameha and Vanta
Basic multi-device handoff hooks
Strong confinement on the Fedora bootc foundation
Related
Project
Role
vanta
Desktop; sends intent, shows results
chameha
Voice and personality
objectspace
Execution world model
byngonet
Inter-node transport
sentinel
Health of AION process
fedora-base
OS substrate
cognitive-runtime
Historical precursor
License
To be decided. All rights reserved until a license is published.
Sorabbyngo · The Rhythm of Digital Life
Models come and go.
The path from intent to effect must remain yours.
