# Features

Northbourne's main philosophy is that the operation of most CM tools has become too complex, and need to be simplier to use if more people are to start utilising it. It can be a fantastic way to initially provision and manage existing infrastructure if used correctly.

---

<svg height="48" class="octicon octicon-mirror" viewBox="0 0 16 16" version="1.1" width="48" aria-hidden="true"><path fill-rule="evenodd" d="M15.5 4.7L8.5 0l-7 4.7c-.3.19-.5.45-.5.8V16l7.5-4 7.5 4V5.5c0-.34-.2-.61-.5-.8zm-.5 9.8l-6-3.25V10H8v1.25L2 14.5v-9l6-4V6h1V1.5l6 4v9zM6 7h5V5l3 3-3 3V9H6v2L3 8l3-3v2z"></path></svg>
### Context Agnostic

Northbourne is designed to run independent from the endpoint you want to configure (similar to Ansible), without being forced to be a "master" or a "slave". The context driver can be the local shell (Eg. bash, zsh), a remote one (Eg. ssh), or even a custom one for various communication methods.

---

<svg height="48" class="octicon octicon-checklist" viewBox="0 0 16 16" version="1.1" width="36" aria-hidden="true"><path fill-rule="evenodd" d="M16 8.5l-6 6-3-3L8.5 10l1.5 1.5L14.5 7 16 8.5zM5.7 12.2l.8.8H2c-.55 0-1-.45-1-1V3c0-.55.45-1 1-1h7c.55 0 1 .45 1 1v6.5l-.8-.8c-.39-.39-1.03-.39-1.42 0L5.7 10.8a.996.996 0 000 1.41v-.01zM4 4h5V3H4v1zm0 2h5V5H4v1zm0 2h3V7H4v1zM3 9H2v1h1V9zm0-2H2v1h1V7zm0-2H2v1h1V5zm0-2H2v1h1V3z"></path></svg>
### Desired State

Northbourne is based around a desired state architecture. It provides an interface for configuring hosts by connecting the Northbourne agent directly to a Git repository and configuring the hosts – or groups of hosts – with how they should eventually look. It's designed to parse a manifest file `north.yml`, and to understand forward and rollback transactions based on changes to manifest within Git. The idea being configuration rollbacks should be as easy as a `git revert; git commit; git push`.

---

<svg height="48" class="octicon octicon-git-pull-request" viewBox="0 0 12 16" version="1.1" width="48" aria-hidden="true"><path fill-rule="evenodd" d="M11 11.28V5c-.03-.78-.34-1.47-.94-2.06C9.46 2.35 8.78 2.03 8 2H7V0L4 3l3 3V4h1c.27.02.48.11.69.31.21.2.3.42.31.69v6.28A1.993 1.993 0 0010 15a1.993 1.993 0 001-3.72zm-1 2.92c-.66 0-1.2-.55-1.2-1.2 0-.65.55-1.2 1.2-1.2.65 0 1.2.55 1.2 1.2 0 .65-.55 1.2-1.2 1.2zM4 3c0-1.11-.89-2-2-2a1.993 1.993 0 00-1 3.72v6.56A1.993 1.993 0 002 15a1.993 1.993 0 001-3.72V4.72c.59-.34 1-.98 1-1.72zm-.8 10c0 .66-.55 1.2-1.2 1.2-.65 0-1.2-.55-1.2-1.2 0-.65.55-1.2 1.2-1.2.65 0 1.2.55 1.2 1.2zM2 4.2C1.34 4.2.8 3.65.8 3c0-.65.55-1.2 1.2-1.2.65 0 1.2.55 1.2 1.2 0 .65-.55 1.2-1.2 1.2z"></path></svg>
### Automatic Security Pull Requests

It's also designed to understand security vulnerabilities located within your configuration, and to automatically create pull requests to Github-; and Gitlab- based repositories based on known application, package and module CVE's. Similar to how npm provides automatic security PR's to any outstanding known problem versions of packages, the same can be done with Northbourne IaC manifests.

---

<svg height="48" class="octicon octicon-clippy" viewBox="0 0 14 16" version="1.1" width="49" aria-hidden="true"><path fill-rule="evenodd" d="M2 13h4v1H2v-1zm5-6H2v1h5V7zm2 3V8l-3 3 3 3v-2h5v-2H9zM4.5 9H2v1h2.5V9zM2 12h2.5v-1H2v1zm9 1h1v2c-.02.28-.11.52-.3.7-.19.18-.42.28-.7.3H1c-.55 0-1-.45-1-1V4c0-.55.45-1 1-1h3c0-1.11.89-2 2-2 1.11 0 2 .89 2 2h3c.55 0 1 .45 1 1v5h-1V6H1v9h10v-2zM2 5h8c0-.55-.45-1-1-1H8c-.55 0-1-.45-1-1s-.45-1-1-1-1 .45-1 1-.45 1-1 1H3c-.55 0-1 .45-1 1z"></path></svg>
### Manual Command to IaC Conversion
Northbourne is also designed to handle a translation of shell commands to modify the manifest manifest, such that the shell history can be parsed back into the manifest – usually on a manual basis. Imagine a tool that ensured nginx was always installed based on the fact you typed `sudo apt-get install nginx`. Existing configuration files can also be specified to be templated into the repository, with automatic hostname and directory variable extraction.
