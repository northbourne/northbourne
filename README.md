# Northbourne

Northbourne is a context-agnostic configuration management agent written in Rust. It takes the best features of Puppet, Chef and Ansible and combines them into one single unusable program.

[![Build Status](https://travis-ci.com/sifex/northbourne.svg?branch=master)](https://travis-ci.com/sifex/northbourne) ![https://img.shields.io/badge/please-don't%20use%20this-red](https://img.shields.io/badge/please-don't%20use%20this-red)

## Why should I use this?

You shouldn't. Seriously. Northbourne is currently in experimental development phase, looking at designing good abstractions from the outset, trying to understand in-depth what other configuration management software does poorly and to improve on them.

## Features

Northbourne's main philosephy is that the learning curve to use most CM tools has become too complex, and needs to be simplified if more environments are to start utilising it.

### Stateful Manifest

Northbourne provides an interface for configuring hosts by connecting Northbourne directly to a Git repository. It's designed to parse a manifest file `north.yml`, and to understand forward and rollback transactions based on changes to manifest within Git. Rollbacks should be as easy as a `git revert`.

### Manual Configuration to IaC Conversion

It's also designed to handle a translation of shell commands to modify the manifest manifest, such that the shell history can be parsed back into the manifest – usually on a manual basis. 

### Automatic Security Pull Requests

Lastly it's designed to understand security vulnerabilities located within your configuration, and to automatically create pull requests to Github-; and Gitlab- based repositories based on known application, package and module CVE's. Similar to how npm provides automatic security PR's to any outstanding known problem versions of packages, the same can be done with Northbourne IaC manifests.

## Context Agnostic?

Northbourne is designed to run independant from the endpoint you want to configure, without being forced to be a "master" or a "slave". The context driver can be the local shell (Eg. bash, zsh), a remote one (Eg. ssh), or even a custom one for various communication methods.

## Contributing

Northbourne's is currently requesting RFC's / Discussions around different use cases. Please feel free to lodge any issues as discussion points.
