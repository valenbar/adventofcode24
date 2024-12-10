# Check if the required arguments are provided# Check if the required arguments are provided
def main [postfix: string] {
    let source_folder = "./day0/"
    
    # Define the destination folder name
    let destination_folder = ($"./day($postfix)")
    
    # Check if the source folder exists
    if ($source_folder | path exists) {
        # Create the destination folder if it doesn't exist
        if not ($destination_folder | path exists) {
            mkdir $destination_folder
        }
    
        # Copy the contents of the source folder into the destination folder
        cp -r $"($source_folder)/*" $destination_folder
        print "Folder copied to: " + $destination_folder
    } else {
        print "Source folder does not exist."
    }
    # edit name in Cargo.toml
    open $"($destination_folder)/Cargo.toml" | upsert package.name { echo $"day($postfix)" } | save -f $"($destination_folder)/Cargo.toml"
}

