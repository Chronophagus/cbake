# CBake
Utility to generate cmake files and build your projects. 

# Description
This tool is designed to get rid of routine job with CMakeLists.txt files. 
At the current time it can generate one of the project layouts described below, collect sources within it and update CMakeLists.txt automatically.

### Example - Creating a new project:

![Image: Creating a project](https://i.imgur.com/3Iv19aR.png)

Now lets build and run it

![Image: Build and Run](https://i.imgur.com/FFJwznH.png)

After you will add some files on the next build/run command execution cbake will automatically update source list in CMakeLists.txt.

![Image: Update](https://i.imgur.com/CSj1Ezq.png)

### Supported project layouts
<TODO: Describe project layouts!>

# Install 

<TODO: Improve this section>

This tool requires CMake version >=2.8.

1. Ubuntu / Debian
  - Download latest cbake .deb package from repository releases
  - run ```sudo dpkg -i cbake-[version]-[arch].deb```
  - verify the installation: 'cbake --version'
2. Other linux 
  - Download lates cbake binary tarball from repository releases.
  - Unpack executable to the /home/user/bin or /usr/local/bin directory
  - Verify installation ```cbake --version```
3. Other
  - Build from sources.
