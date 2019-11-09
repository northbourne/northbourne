# Features

Northbourne's main philosophy is that the operation of most CM tools has become too complex, and need to be simplier to use if more people are to start utilising it. It can be a fantastic way to initially provision and manage existing infrastructure if used correctly.

### Context Agnostic

Northbourne is designed to run independent from the endpoint you want to configure (similar to Ansible), without being forced to be a "master" or a "slave". The context driver can be the local shell (Eg. bash, zsh), a remote one (Eg. ssh), or even a custom one for various communication methods.

### Desired State

Northbourne is based around a desired state architecture. It provides an interface for configuring hosts by connecting the Northbourne agent directly to a Git repository and configuring the hosts – or groups of hosts – with how they should eventually look. It's designed to parse a manifest file `north.yml`, and to understand forward and rollback transactions based on changes to manifest within Git. The idea being configuration rollbacks should be as easy as a `git revert; git commit; git push`.

### Automatic Security Pull Requests

It's also designed to understand security vulnerabilities located within your configuration, and to automatically create pull requests to Github-; and Gitlab- based repositories based on known application, package and module CVE's. Similar to how npm provides automatic security PR's to any outstanding known problem versions of packages, the same can be done with Northbourne IaC manifests.

### Manual Command to IaC Conversion

Northbourne is also designed to handle a translation of shell commands to modify the manifest manifest, such that the shell history can be parsed back into the manifest – usually on a manual basis. Imagine a tool that ensured nginx was always installed based on the fact you typed `sudo apt-get install nginx`. Existing configuration files can also be specified to be templated into the repository, with automatic hostname and directory variable extraction.