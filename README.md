# dig

## Overview

Dig is a command-line tool to search files for string and output this to the console.

## Usage

Search strings are not case-sensitive.

```powershell
.\dig.exe "C:\Path\To\File\Here.txt" "StringToSearch"
```

Example Output:
```
.\dig.exe A:\name\dig\files\example_logfile.log "08:51:06 TRACE"
[A:\name\dig\files\example_logfile.log] 03/22 08:51:06 TRACE  :...read_physical_netif: Home list entries returned = 7
[A:\name\dig\files\example_logfile.log] 03/22 08:51:06 TRACE  :..entity_initialize: interface 129.1.1.1, entity for rsvp allocated and initialized
[A:\name\dig\files\example_logfile.log] 03/22 08:51:06 TRACE  :..entity_initialize: interface 9.37.65.139, entity for rsvp allocated and 
[A:\name\dig\files\example_logfile.log] 03/22 08:51:06 TRACE  :..entity_initialize: interface 9.67.100.1, entity for rsvp allocated and initialized
[A:\name\dig\files\example_logfile.log] 03/22 08:51:06 TRACE  :..entity_initialize: interface 9.67.101.1, entity for rsvp allocated and initialized
[A:\name\dig\files\example_logfile.log] 03/22 08:51:06 TRACE  :..entity_initialize: interface 9.67.116.98, entity for rsvp allocated and 
[A:\name\dig\files\example_logfile.log] 03/22 08:51:06 TRACE  :..entity_initialize: interface 9.67.117.98, entity for rsvp allocated and
```