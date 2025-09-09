#simple class 

echo "Hello, $(whoami)! Welcome to the Shell Script Demo."


echo "Current date and time: $(date)"


echo -e "\nHere are the files in your current directory:"
ls -lh


echo -e "\nEnter a filename to check and display its contents:"
read filename


if [ -f "$filename" ]; then
    echo -e "\n File '$filename' found. Displaying contents:\n"
    cat "$filename"
else
    echo -e "\n File '$filename' does not exist."
fi
