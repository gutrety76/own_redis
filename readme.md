# HOLY REDIS

Database written in rust for fun. 

To connect to database simply connect to ```127.0.0.1 6379```

_Commands_:

    Get value by key*: ```GET <key>```
    Set value by key: ```SET <key> <value> <expired>```
       - if you won't set expired state. It will be set to 1 day
