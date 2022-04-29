Rust-Unix-Shell

Implementation of Unix Shell in Rust. Following are the features to be impelemented in the Unix Shell with Rust:
a. cmd_history: Displays the history of commands entered in the command line.

b. rmallexn(remove all except n): Remove all the files and directories in the directory path mentioned in the argument except the file mentioned.

c. listDir [-l][-a][-tree][-color] <directory>: List the contents of the directory with optional arguments.

d. rev_search(Reverse search and execute): Search in the history to check if the substring entered matches any entry and accordingly execute it

e. sortbytype (Sort by type): This command is used to combine multiple files of similar pattern or type and it will add those files into a one separate sub directory.

f. pipe operator(|): This command is used to combine multiple command and run them sequentially. 

g. input redirection (<): This command redirects the input from a file and passes it to a command as parameter insted of passing the parameter from standard input.

h. output (>) : Saves the output of the command into a file rather than displaying it on the console.

i. change directory (cd) : This command is used to change the directory and travel back and forth into the directories.

j. quit: Use quit to get out of rust command line.

# Crates
1. json = 0.12.4
2. colored = 2.0.0
3. chrono = 0.4
4. permissions = 0.3.0
5. is_executable = 1.0.1
6. libc = 0.2.122
7. termion = 1.5.6
8. pbr = "1.0.4"
9. fs-utils = "1.1.4"
10. rustc-serialize = "0.3.24"

# cmd_history
1. Track down all the commands been entered onto the command line.
2. Displays the history of commands entered.

cmd usage: cmd_history

# listDir
1. The list command will accept a variety of parameters.
2. The parameters involves:

    a. -l : List the files and directories in the path mentioned as the argument.
    
    b. -a: List all the files including the hidden files, symbolic links.
    
    c. -tree: Show the hierarchy of all the files and directory inside the current directory.
    
    d. -color: Display the files in different colors as per their file type

cmd usage: listDir [-l] [-a] [-tree] [-color] \<directory\>
1. listDir -l \<directory\>
2. listDir -l -color \<directory\>
3. listDir -tree \<directory\>
4. listDir -tree 
5. listDir -a \<directory\>
6. listDir -a 

# rmallexn
Deletes the file and folder in a directory except the one passed as argument to the command

Sample syntax: 
1. rmallexn test/abc/def 
The above command will delete all the files and directories in test/abc/ and keep only test/abc/def
    
2. rmallexn test/xyz/file1.txt
The above command will delete all the files and directories in test/xyz/ and keep only test/xyz/file1.txt

The file or the directory to keep should not have whitespaces in the name.

# rev_search
Searches the history list for the existence of the command and the new feature that is added is that the user gets to correct his search and the list will start again from the tail.

cmd_usage: rev_search

Once the command is entered, a raw console will be initiated. There the user can enter the command or a substring of a command he wishes to execute. The result of the search will be shown besides the substring.
If the user is successful in finding the command he wishes to execute, simply press enter and the command will be executed. 

To simplify searches, this API also has provided the feature of provided the functionality of searching ahead in the list if there are multiple entries in the list that has the same substring.

Ctrl('r') is the way to invoke that functionality.

# General attributes

1. | (pipe operator)
The above command will combine multiple command and run them sequentially.

Sample Syntax:

listDir -a | listDir -l

2. < (input redirection)
The above command redirects the input parameter to a command from a file.

Sample Syntax:

listDir -a < input.json

This will take the input for listDir -a from the input.json file and run the listDir command.

3. > (File output)

The above command saves output of any command into a file.

Sample Syntax:

command > \<file_name\> 

Example : 

listDir -l -color > output.txt

The above command will save result of listDir -l -color into the file output.txt

# sortbytype

1. Sort by type:

cmd usage: sortbytype -t extension \<directory_name\>

Above command will move all files with given extension to given sub-directory name. It will add files in sub directory if it already exists otherwise it will creat a new sub directory. 

If there are no matching files with given extension it will return an error and it will not create any new sub directory.

2. Sort by name:

cmd usage: sortbytype -n file_sub_string \<directory_name\>

sortbytype -n will take file sub string and move all files with containing sub string to sub directory. If sub directory does not exist then it will create a new sub directory. 

If there are no matching files with given file sub string then it will return an error and will not create any sub directory.

# Change Directory (cd)

Change directory is used to traverse through the directories.

Sample Syntax:

cd \<directory_name\>

This command will shift forward from current working directory to directory name.

cd ..

This command will shift backward from current working directory.

Example: cd dummy-files

This command will change working directory from current directory to the dummy-files directory. It also checks if the directory name is a valid directory.
