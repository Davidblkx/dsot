# How to sync data between instances

In dsot we can have multiple machines using the same data. To keep data in sync, we use a journaling system.
The journal is a list of actions that have been performed on the data. Each action is stored a KV database.
When a new instance is created, it can start from an empty database or use a snapshop of an existing database.
Then any change will be recorded in the journal. The journal can be used to replay the changes on another instance.
The database will also have a table to store the last journal entry that has been applied to each entity.

## Journal Entry

- entry_id: uuid7
- action: create | update | delete
- entity: "entity name"
- entity_id: "entity id"
- body: JSON

By using the uuid7, we can ensure that the journal entry is unique and that the order of the entries is preserved.
So if we try to sync two instances, we can just replay the journal entries in order.

## Syncing

Each instance has it's own ID generated at creation. In the first sync between two instances all
journal entries will be sent to the other instance. Each instance saves the last journal entry that has been applied
to each entity by using the instance id as key.

## Clean up

To keep the journal entries from growing too large, we can periodically clean up the entries. We can do this by deleting all entries that are older than the oldest entry that has been applied to each entity. This can cause some data loss, but it is a tradeoff between data loss and storage space. So we need to have an option for an instance to just start from scratch and get all the data from another instance.

# Technical Details

  For SQLite database: library.db:

- It will have a table SYNC_STATUS with hardcoded entity id and last journal entry id.
- We will have a redb database file called library.jrn
- It will be used to store the journal entries.
- There will be a bucket for each entity.
