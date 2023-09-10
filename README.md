# clerk
#### A multi-platform todo-list cli-application

## Usage
The use of this application is pretty simple. You should first download the latest release.

If you're on windows you could just install the clerk.exe and your pretty much done.

If you're on linux or on mac you should first download the given .zip or .tar.gz and extract it. Then
you should compile it by using 'cargo build --release'. If you don't have cargo or rustc installed 
you should first do that. Just go to the offical rust site and follow their instructions.

That's it, you should be able to run the file now. But if you want easy access to your list you should
first add it to your PATH variables. 

After you've done that, close your terminal and fire it back up again.
You should be able to just type 'clerk' and get greeted by an empty list.

To see which options you have and which commands are available, just type the '-h' flag after a command.
Like this: 
```bash
$ clerk -h
# returns all the commands you could use

$ clerk
# returns your todolist 

```
## Example 
How to add a task with a subtask and flag it as doing 
```bash
$ clerk add "Main task"
# add the main task

$ clerk add "Sub task of the Main task" -s 0
# -s flag marks the input as a subcommand of index 0
# usage: clerk add <StringTask> -s <IndexOfMaintask>

$ clerk mark 0 0 -d
# marks the subtask 0 of maintask 0 as doing

$ clerk mark 0 -d
# marks the maintask as doing
```
Remove a main task or subtask
```bash
$ clerk mark 0 0 -r
# removes the subcommand 0 of main task 0
```

## Config
These are the default locations for different operating systems
```
Linux:   /home/alice/.config/clerk
Windows: C:\Users\Alice\AppData\Roaming\meloencoding\clerk
macOS:   /Users/Alice/Library/Application Support/dev.meloencoding.clerk
```

```toml
# note: toml doesn't support comments so remove everything after every '#'
[settings]
local = false # use local_location
use_unicode = false # set to true if you experience weird characters in your terminal
color_blind = true # if you don't like the colors or if experience weird characters in your terminal, set to true
page_size = 5 # set the page size when you run 'clerk'

[locations]
config_dir = '' # gets set when you first run the program. you could change it to a custom location 
local_location = '' # gets set when you first run the program. you could change it to a custom location 
remote_location = "" # you need to set this your self to the location of your compatible api

# if you use some sort of validation in your api, you could set these to check validation in your api
[api]
remote_key = "" 
app_id = ""
app_key = ""
```

## External use
If you want to use the external function you'll need to have a compatible API. In the source there is a file called 'clerk.ts'. If you 
have a expressjs server you could just follow the instructions in the file. 

If not, 

you just need only one endpoint that has some sort of switch case for a string called "/show" and "/set". Both are self 
explaining in function. The show function should read out your data file and the set function should rewrite the entire file. 
Both need to return a json body with a valid key. Of course the show function should also return the value of the recently read
file in an data key. 

For reference, check out 'clerk.ts'.

After you've succesfully started your api you should now add it to the 'remote_location' option in the config file. Check the 'Config' 
paragraph for the location of your OS. Also don't forget to change 'local' to false in your config.

For questions, please open an issue on this project.
