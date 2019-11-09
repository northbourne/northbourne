<p align="center">
  <img width="116" height="117" src="https://github.com/sifex/northbourne/raw/master/docs/images/logo.png">
</p>

<h1 align="center">Northbourne</h1>

<p align="center">A context-agnostic desired-state configuration management agent written in Rust<br /><i>Taking the best features of Puppet, Chef and Ansible and combines them into one single unusable program.</i></p>

<p align="center"><a href="https://travis-ci.com/sifex/northbourne"><img alt="Travis (.com)" src="https://img.shields.io/travis/com/sifex/northbourne?logo=travis"></a> <a href="https://github.com/sifex/northbourne/graphs/contributors"><img src="https://img.shields.io/github/contributors/sifex/northbourne?color=blue&logo=github" alt="GitHub contributors"></a> <a href="https://github.com/sifex/northbourne/stargazers"><img alt="GitHub stars" src="https://img.shields.io/github/stars/sifex/northbourne?color=yellow&logo=github"></a> <img src="https://img.shields.io/github/downloads/sifex/northbourne/total?logo=github" alt="GitHub All Releases"> <img src="https://img.shields.io/badge/please-don&#39;t%20use%20this-red" alt="https://img.shields.io/badge/please-don&#39;t%20use%20this-red"> <a href="https://github.com/sifex/northbourne/blob/master/LICENSE"><img alt="GitHub license" src="https://img.shields.io/github/license/sifex/northbourne"></a></p>

## Why should I use this?

You shouldn't. Seriously. Northbourne is currently in experimental development phase, looking at designing good abstractions from the outset, trying to understand in-depth what other configuration management software does poorly and to improve on them.

## Features

Northbourne's main philosophy is that the learning curve to use most CM tools has become too complex, and needs to be simplified if more environments are to start utilising it.

### Desired State

Northbourne is based around a desired state architecture. It provides an interface for configuring hosts by connecting Northbourne agent directly to a Git repository and configuring the hosts – or groups of hosts – how they should look. It's designed to parse a manifest file `north.yml`, and to understand forward and rollback transactions based on changes to manifest within Git. Rollbacks should be as easy as a `git revert`.

### Manual Configuration to IaC Conversion

It's also designed to handle a translation of shell commands to modify the manifest manifest, such that the shell history can be parsed back into the manifest – usually on a manual basis. Imagine a tool that ensured nginx was always installed based on the fact you typed `sudo apt-get install nginx`. Existing configuration files can also be specified to be templated into the repository, with automatic hostname and directory variable extraction.

### Automatic Security Pull Requests

Lastly it's designed to understand security vulnerabilities located within your configuration, and to automatically create pull requests to Github-; and Gitlab- based repositories based on known application, package and module CVE's. Similar to how npm provides automatic security PR's to any outstanding known problem versions of packages, the same can be done with Northbourne IaC manifests. 

## Context Agnostic?

Northbourne is designed to run independent from the endpoint you want to configure, without being forced to be a "master" or a "slave". The context driver can be the local shell (Eg. bash, zsh), a remote one (Eg. ssh), or even a custom one for various communication methods. 

## Contributing

Northbourne's is currently requesting RFC's / Discussions around different use cases. Please feel free to lodge any issues as discussion points.
