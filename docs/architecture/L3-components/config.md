# DSOT Configuration

DSOT can be multiple kinds of applications (cli, mobile, desktop gui) meaning that we could have different config needs even for the same user.
In this document we are going to describe how configuration is storead, read and updated in way that support the multiple use cases.

## bakunin_config

Configs are built on top of the crate bakunin_config, it will allow us to load configuration from multiple sources and give users the flexability to store configs in multiple places
