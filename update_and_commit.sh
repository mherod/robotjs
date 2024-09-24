#!/bin/bash

# Update .gitignore
echo "Creating/Updating .gitignore"
cat << EOF > .gitignore
/target
**/*.rs.bk
Cargo.lock
node_modules
*.node
EOF

# Add new files
echo "Adding new files to git"
git add .cargo/ .circleci/ Cargo.toml build.rs src/lib.rs
git add tests/lib.rsgit add index.d.ts index.js package.json

# Add yarn.lock if it exists
if [ -f yarn.lock ]; then
    echo "Adding yarn.lock"
    git add yarn.lock
fi

# Show status
echo "Current git status:"
git status

# Prompt for commit
read -p "Do you want to commit these changes? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]
then
    # Commit changes
    echo "Committing changes"
    git commit -m "Implement RobotJS using Rust with Node.js bindings"

    # Prompt for push
    read -p "Do you want to push these changes to the remote repository? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]
    then
        echo "Pushing changes to remote repository"
        git push origin master
    else
        echo "Changes committed but not pushed"
    fi
else
    echo "Changes not committed"
fi

echo "Script completed"