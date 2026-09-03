# Pet behavior runtime

`robotd` API v17 makes the robot advertise what it can execute instead of making the App
guess from a product-wide list. The `robot.subscribe` result now contains:

- `runtime`: `hardware` for `robotd` (`simulator` is reserved for the digital twin);
- `available_behaviors`: behavior ids that are executable by this running release and its
  configured policy/voice bundle.

The App and Brain may show a friendly product catalogue, but they must intersect it with
this runtime list before dispatch. Missing ids are unavailable, not silently emulated.

Four small expressions are deterministic standing-policy trajectories and therefore do
not require separate ONNX files: `head_tilt`, `curious_scan`, `greet`, and `invite_play`.
Their head/body commands remain inside the trained command ranges and pass through the
normal observation, inference, joint-limit, and safety pipeline. `greet` and `invite_play`
use voice-bank sounds when a bank is installed, but the pose still works without audio.

Dynamic pet motions such as approach, retreat, nuzzle, stretch, and bounce are deliberately
not in the hardware capability list yet. They enter it only after a policy has completed
simulation regression and is included in a robot release.

`robot.stop`, `robot.enable {on:false}`, and `robot.relax` cancel queued and active transient
skills. A manual non-zero velocity also interrupts a deterministic expression. This makes
the App's emergency sequence (`robot.stop`, then disable) authoritative even if an ONNX
skill or pet expression was already running.

