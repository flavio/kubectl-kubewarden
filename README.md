# kubectl-kubewarden

> ⚠️ ⚠️ **WARNING:** this plugin is an experimental POC meant to demonstrate
> how kubectl plugins can be written using WebAssembly and WASI. It currently
> offers really limited features that are available only with an
> experimental integration of the Kubernetes API server and Kubewarden.

This is an experimental kubectl plugin that interacts with a Kubewarden instance
deployed on top of a Kubernetes cluster.

This plugin is written using WebAssembly and WASI, it can be used via the
[krew-wasm](https://github.com/flavio/krew-wasm) plugin manager.

## Prerequisites

* A Kubewarden stack
* A working kubeconfig file
* A vanilla version of `kubectl`
* Latest version of `krew-wasm` installed

## Installation

Simply perform the following command:

```console
krew-wasm pull ghcr.io/flavio/krew-wasm-plugins/kubewarden:latest
```

As reported by `krew-wasm pull`, make sure to add `$HOME/.krew-wasm/bin` to your
`$PATH` so that `kubectl` can find the `kubectl-kubewarden` plugin.

## Usage

Like any other regular kubectl plugin, just invoke:

```console
kubectl kubewarden --help
```

### Events

The plugin can list the Kubernetes events with `reason` set to `ValidationRejection`:

```
$ kubectl kubewarden events
Found 2 events
╔═════════════════════╦═════════╦════════════════════════════════════════╦════════════════════════════════════════╦═══════════════════════════╦═══════════════════════════╗
║        Source       ║   Type  ║                 Message                ║             Involved object            ║         First seen        ║         Last seen         ║
╠═════════════════════╬═════════╬════════════════════════════════════════╬════════════════════════════════════════╬═══════════════════════════╬═══════════════════════════╣
║ kubewarden-embedded ║ Warning ║ privileged: User 'minikube-user' canno ║ ObjectReference { api_version: Some("v ║ 2022-04-21T14:36:34+00:00 ║ 2022-04-22T10:34:40+00:00 ║
║                     ║         ║ t schedule privileged containers       ║ 1"), field_path: None, kind: Some("Pod ║                           ║                           ║
║                     ║         ║                                        ║ "), name: None, namespace: None, resou ║                           ║                           ║
║                     ║         ║                                        ║     rce_version: None, uid: None }     ║                           ║                           ║
╠═════════════════════╬═════════╬════════════════════════════════════════╬════════════════════════════════════════╬═══════════════════════════╬═══════════════════════════╣
║ kubewarden-embedded ║ Warning ║ trusted-repos: not allowed, reported e ║ ObjectReference { api_version: Some("v ║ 2022-04-21T14:52:25+00:00 ║ 2022-04-22T10:34:20+00:00 ║
║                     ║         ║ rrors: registries not allowed: docker. ║ 1"), field_path: None, kind: Some("Pod ║                           ║                           ║
║                     ║         ║ io                                     ║ "), name: None, namespace: None, resou ║                           ║                           ║
║                     ║         ║                                        ║     rce_version: None, uid: None }     ║                           ║                           ║
```

> **Note well:** This type of events is not emitted by a regular
> Kubewarden installation, this is currently done only by an
> experimental integration of the Kubernetes API server and
> Kubewarden.
