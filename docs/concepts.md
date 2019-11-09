# Concepts

## Transactions

Each step in applying a certain configuration is broken down into atomic actions or steps, defining one action to be performed on a system at a time. Each transaction is made up of an operation with parameters, a predicate, and a rollback. Transactions are specified in a manifest file, defined later in the report.

## Operators

Operators are defined as handlers for operations. They are the fundamental building blocks for creating a transaction. Our core operators are File, Services, Permissions, Packages, and Templates. Each operator has a set of pre-defined parameters, such that each interacts with the operating system to accomplish the goal of the operation. (Eg. A File operator can be used for anything pertaining to adding, moving, renaming or removing files from the operating system).

## Rollbacks

Rollbacks are the opposite of operations, and are mandatory within every transaction within a module. Most rollback operations can be inferred, or it can be explicitly defined, but there must always exist a rollback operation to reverse a transaction. This design decision creates a much more simplified understanding if a module is required to be undone.

## Contexts

Contexts can be defined as interfaces for operators. They implement, at a lower level, how each operation interfaces with the operating system, either locally, or remotely. It also defines support for each of the core operators (Files, Services, Permissions, Packages, Templates etc).

## Predicates

Predicates – as they are defined in the logical sense of the word – is defined as: “something which is affirmed or denied concerning an argument of a proposition”. For this reason, the concept was created within this configuration management tool that; any operation set to be actioned must satisfy a binary predicate, either to commence the action if required, or to not complete the action if it’s already satisfied.

There are many different definitions of predicates for a variety of different use cases, such as whether a package is installed or not, conducting regular expression over files, ensuring files are present or absent, ensuring certain types of files have certain types of permissions etc.

## Security Assurance

For the security assurance feature set of the configuration management tool, we can utilise the predicates mentioned above with no operation to be completed on the transaction as a whole, either forward or rollback. We can think of this as checks on a system to ensure that it complies with the most basic form of a certain security control.

An example of this could be to see if a server has configuration around complying to a certain standard, such as the Australian Signals Directorate’s ISM Essential 8 (\acrshort{e8}) standard \cite{noauthor_essential_nodate}. The \acrshort{e8} module could ensure a system adhered to the standard, and report back if there existed a breach of that standard.

## Manifest File, Module DSL & Templating Engine

Placing the above concepts into a code-based form is quite difficult to image and execute, especially one that is robust enough to meet our hypothesis’ requirement for simplicity and elegance. Three types of files for this tool were required to be constructed.

The first – the Manifest File – is the main interface for using the configuration management tool as the user. It explicitly states which modules are applied to which server, as well as which module parameters are to be overridden from the defaults. This can be as simple as specifying that a web server module is to be applied to a server, with the default web root location to be changed out from the default.

The second – the Module spec File – is a \acrfull{dsl_acr} based file, listing of all transactions, operations and workflows that are included within a module. It specifies the exact “how” to apply certain operations, as well as specifies any parameters that can be overridden by the manifest, specifying defaults when they’re not.

The third – Templating Engine – exist as a way for module files to place parameters into configuration files before writing them to the system. They can also implement any conditional logic, loops and

A few \acrlong{dsl_acr}s were trialed in place for the manifest configuration, the module file specification, and the templating engine.

\textbf{YAML} was chosen for the manifest file, as it lends itself to a very readable, simple hierarchical structure, with no added complexities of variables, logic or braces.

\textbf{OVER}\cite{s_over:_2019} was chosen for the Module spec files due to its versatility, safety, readability, and features such as global variables that can be used later on throughout the module specification.

\textbf{Liquid}\cite{shopify_liquid_nodate} was chosen for the templating engine, as a very well supported rust-compatible templating language, with very good support for different iteration, variable, comment and filter features.

## Workflows & Wrapping

In a lot of cases, services are required to be refreshed or restarted after configuration files have been updated. Some of the indicated issues with the existing configuration management tools Puppet Open Source & Puppet Enterprise was that the addition of pre-action and post-action hooks attached to any operation made it incredibly difficult to understand the order of operations in which action executed.

For this reason – to aid transparency for developers – transactions can be “wrapped” into a workflow, such as a “refresh” workflow within a module. Once all of the transactions in a workflow have been completed, then post-actions hooks can be fired off to refresh the overall module.

These post-actions hooks are not designed to mutate any configuration, files or packages, they should only be designated for actions that refresh existing configurations if need be. They can also just be used as a grouping for common sets of transactions.

## Operation & Flow

A design decision taken in the interest of security – also influenced by my employment – is by lowering the barrier to entry to the software, the more consideration is required to be given toward ensuring secure configuration by default, as less advanced users are to be using the software.

A methodology of omnidirectional communication was established, such that application cannot be controlled by an outside actor, and could only “pull” new information from already established trusted sources.

\begin{figure}[H]
\includegraphics[width=7.5cm]{images/flow.png}
\centering
\caption{Operation & Flow Diagram}
\label{fig:flow_diagram}
\end{figure}

On initial setup, the repository containing the manifest is issued to the agent, which fetches the manifest periodically (Eg. Every 30 minutes) and completes a system evaluation based on each applicable modules’ predicates and workflows. Once that’s been completed, the run results are sent to wherever the specified reporting destination is set, within the manifest file.

\subsubsection{VCS & Manifest Download}

The manifest will be a file located within any Git repository – itself dedicated to housing the manifest as well as any bespoke modules – to describe the outline of that project’s infrastructure configuration. The manifest must be called “north.yml”. The software has reference to this in the initial setup, and can take the latest version

One added benefit to using a \acrfull{vcs} being the history of a configuration's state can be rolled both forwards and backwards based on the commits of the repository. Most \acrlong{cm} tools recommend using \acrshort{vcs} to house configuration, but having an inherit understanding of their existence allows for much more native, simple and elegant behaviour of the \acrlong{cm} system as a whole.

\subsubsection{Reporting}

Reporting the results of a run is optional, but highly recommended. The supported driver in the final developed app should be webhook, email, slack, as well as others, to report back to the system owner as to the status of any issues arising from the configuration evaluation phase.