for path in "$@"; do 
    PATH=$PATH:$path    
done
echo "export PATH=$PATH" >> ~/.bashrc