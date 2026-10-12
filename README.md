# AION

**AI runtime and control plane for Sorabbyngo OS**

AION is not an AI model.  
It is the system layer that turns intent into safe, mediated action.

Models reason.  
AION decides, routes, remembers enough to continue, and executes through the rest of the OS.

---

## Role

```text
User
  ↓
Chameha / Vanta          (conversation & direct UI)
  ↓ intent
AION                     (this component)
  ├── select backend (local model / API model / remote node)
  ├── assemble context
  ├── validate tool calls
  └── mediate actions
  ↓
ObjectSpace & system
AION is responsible for:
Receiving intent (from Chameha, Vanta, or other clients)
Planning and decomposing tasks
Choosing which model backend to use
Assembling the right context for that backend
Validating and authorizing tool calls
Executing actions only through mediated APIs (ObjectSpace / typed actions)
Coordinating with other nodes when needed (via Byngonet)
Keeping actions observable, permissioned, and undoable where possible
AION is not responsible for:
Being the voice or personality (that is Chameha)
Owning the full long-term memory database (Context/Memory service)
Replacing the mesh transport (that is Byngonet)
Supervising every OS process (that is systemd + Sentinel)
Hard-coding any particular model vendor
Design principles
Runtime, not model — Models are backends. They can be replaced as better ones appear.
One door for actions — GUI, voice, and automation all go through the same mediation path.
Direct path still works — The desktop remains usable if AION is limited or offline.
Local-first — Core planning and local backends work without the network.
Least privilege — Models never receive raw system control. They propose; AION mediates.
Multi-device capable — AION can target another node through Byngonet without the user managing IPs.
Stay a control plane — Do not absorb voice pipelines, full memory storage, or init-system duties.
Backends
AION treats all intelligence providers uniformly:
Backend type
Examples
How AION uses it
Local system models
Pre-installed or user-installed local weights
Default offline path
Containerized servers
Ollama, vLLM, etc.
Registered HTTP/API backends
Cloud APIs
User-supplied API keys
Opt-in, privacy-filtered
Remote AION nodes
Other machines on Byngonet
For continuation / offload
Registration is capability-based. AION selects backends by task needs, policy, and availability — not by hard-coded vendor logic.
Context and memory
AION needs rich context. It should not become the sole memory store.
Working context — assembled per request for the chosen backend
Project / chat memory — owned by a Context/Memory service that AION queries and updates
Context switching — AION decides when to switch; the memory service performs the swap cleanly
This keeps AION focused while still supporting long projects and multi-device continuation.
Safety and mediation
Every model output that would change the system passes through AION:
Parse structured tool calls / plans
Check permissions and policy
Confirm with the user when required
Execute via ObjectSpace / typed action API
Log and support undo where possible
No backend — local or cloud — receives direct filesystem, device, or kernel control.
Relationship to other components
Component
Relationship
Chameha
Sends user intent; receives natural-language results
Vanta
UI and spatial surface; can also send direct actions
ObjectSpace
World model and execution target for mediated actions
Byngonet
Transport and discovery for multi-device AION coordination
Sentinel
Monitors AION health and recovery
CML (if retained)
Optional safety/privacy filter on outbound cloud calls
What changed from earlier research
Older work embedded planning inside Chameha and used Byngox as a multi-agent cognitive core.
Current direction:
AION is the standalone runtime / control plane
Byngox is superseded as the primary intelligence backend
Models are user-choosable and replaceable
Chameha is conversation and personality only
Earlier prototypes remain useful reference, not the final architecture.
Current status
Design and realignment phase.
Goals for the near term:
Stable intent → plan → mediate → act pipeline
Uniform backend registration (local + API)
Clean boundary with Chameha and ObjectSpace
Basic multi-device handoff via Byngonet
Strong confinement on Linux foundations
Non-goals
Becoming a general multi-agent product platform
Replacing systemd or the desktop compositor
Locking users to one model vendor
Requiring cloud connectivity for basic use
Related repositories
Repository
Role
vanta
Spatial desktop
chameha
Voice & personality
objectspace
Objects & spaces
byngonet
Inter-node communication
sorabbyngo-os
Main OS
cognitive-runtime
Earlier related research
License
To be decided.
All rights reserved until a license is published.
Sorabbyngo
The Rhythm of Digital Life
