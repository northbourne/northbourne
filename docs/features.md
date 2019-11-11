# Features

Northbourne's main philosophy is that the operation of most CM tools has become too complex, and need to be simplier to use if more people are to start utilising it. It can be a fantastic way to initially provision and manage existing infrastructure if used correctly.

### Context Agnostic

Northbourne is designed to run independent from the endpoint you want to configure (similar to Ansible), without being forced to be a "master" or a "slave". The context driver can be the local shell (Eg. bash, zsh), a remote one (Eg. ssh), or even a custom one for various communication methods.

### Desired State

Northbourne is based around a desired state architecture. It provides an interface for configuring hosts by connecting the Northbourne agent directly to a Git repository and configuring the hosts – or groups of hosts – with how they should eventually look. It's designed to parse a manifest file `north.yml`, and to understand forward and rollback transactions based on changes to manifest within Git. The idea being configuration rollbacks should be as easy as a `git revert; git commit; git push`.

---

<svg height="64" class="octicon octicon-git-pull-request" viewBox="0 0 12 16" version="1.1" width="48" aria-hidden="true"><path fill-rule="evenodd" d="M11 11.28V5c-.03-.78-.34-1.47-.94-2.06C9.46 2.35 8.78 2.03 8 2H7V0L4 3l3 3V4h1c.27.02.48.11.69.31.21.2.3.42.31.69v6.28A1.993 1.993 0 0010 15a1.993 1.993 0 001-3.72zm-1 2.92c-.66 0-1.2-.55-1.2-1.2 0-.65.55-1.2 1.2-1.2.65 0 1.2.55 1.2 1.2 0 .65-.55 1.2-1.2 1.2zM4 3c0-1.11-.89-2-2-2a1.993 1.993 0 00-1 3.72v6.56A1.993 1.993 0 002 15a1.993 1.993 0 001-3.72V4.72c.59-.34 1-.98 1-1.72zm-.8 10c0 .66-.55 1.2-1.2 1.2-.65 0-1.2-.55-1.2-1.2 0-.65.55-1.2 1.2-1.2.65 0 1.2.55 1.2 1.2zM2 4.2C1.34 4.2.8 3.65.8 3c0-.65.55-1.2 1.2-1.2.65 0 1.2.55 1.2 1.2 0 .65-.55 1.2-1.2 1.2z"></path></svg>

### Automatic Security Pull Requests

It's also designed to understand security vulnerabilities located within your configuration, and to automatically create pull requests to Github-; and Gitlab- based repositories based on known application, package and module CVE's. Similar to how npm provides automatic security PR's to any outstanding known problem versions of packages, the same can be done with Northbourne IaC manifests.

### Manual Command to IaC Conversion

Northbourne is also designed to handle a translation of shell commands to modify the manifest manifest, such that the shell history can be parsed back into the manifest – usually on a manual basis. Imagine a tool that ensured nginx was always installed based on the fact you typed `sudo apt-get install nginx`. Existing configuration files can also be specified to be templated into the repository, with automatic hostname and directory variable extraction.
