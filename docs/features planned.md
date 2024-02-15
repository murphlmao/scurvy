
    # network
    network_group = parser.add_argument_group('Network')
    parser.add_argument("mac", action='store_true') # grab mac address
    parser.add_argument("ip", action='store_true') # grab primary ip address
    parser.add_argument("ipc", action='store_true') # active net info: ip, gateway, subnet, dns, wired/wirless
    parser.add_argument("nets", action='store_true') # network stack

    # TODO: add set ip, dns, gateway, subnet, etc

    # docker
    docker_group = parser.add_argument_group('Docker')
    parser.add_argument("dps", action='store_true') # docker ps -a
    parser.add_argument("dpsa", action='store_true') # docker ps -a
    parser.add_argument("dl") # docker logs <container id>
    parser.add_argument("diprune", action='store_true') # docker prune images

    # os
    os_group = parser.add_argument_group('OS')
    parser.add_argument("os", action='store_true') # operating system
    parser.add_argument("osv", action='store_true') # operating system version
    parser.add_argument("osk", action='store_true') # operating system kernel
    parser.add_argument("osn", action='store_true') # operating system name
    parser.add_argument("osu", action='store_true') # operating system uname
    parser.add_argument("osm", action='store_true') # operating system machine
    parser.add_argument("osr", action='store_true') # operating system release
    parser.add_argument("osb", action='store_true') # operating system build

    # hardware
    hw_group = parser.add_argument_group('hardware')

    ## cpu
    parser.add_argument("cpu", action='store_true') # cpu
    parser.add_argument("cpuc", action='store_true') # cpu count
    parser.add_argument("cpup", action='store_true') # cpu percent
    parser.add_argument("cpuf", action='store_true') # cpu freq
    parser.add_argument("cpum", action='store_true') # cpu max freq
    parser.add_argument("cpumin", action='store_true') # cpu min freq
    parser.add_argument("cpus", action='store_true') # cpu stats
    parser.add_argument("cpua", action='store_true') # all of the above

    ## memory
    parser.add_argument("mem", action='store_true') # memory
    parser.add_argument("memt", action='store_true') # memory total
    parser.add_argument("memhz", action='store_true') # memory frequency
    parser.add_argument("memu", action='store_true') # memory used
    parser.add_argument("memf", action='store_true') # memory free
    parser.add_argument("mems", action='store_true') # memory stats
    parser.add_argument("mema", action='store_true') # all of the above

    ## disk
    parser.add_argument("disk", action='store_true') # disk <disk name>
    parser.add_argument("disks", action='store_true') # disks on the system
    parser.add_argument("disku", action='store_true') # disk used
    parser.add_argument("diskf", action='store_true') # disk free
    parser.add_argument("diskt", action='store_true') # disk total
    parser.add_argument("diskp", action='store_true') # disk percent
    parser.add_argument("diska", action='store_true') # all of the above for all disks

    ## gpu
    parser.add_argument("gpu", action='store_true') # gpu
    parser.add_argument("gpuc", action='store_true') # gpu count
    parser.add_argument("gpup", action='store_true') # gpu percent
    parser.add_argument("gpuf", action='store_true') # gpu freq
    parser.add_argument("gpum", action='store_true') # gpu max freq
    parser.add_argument("gpumin", action='store_true') # gpu min freq
    parser.add_argument("gpus", action='store_true') # gpu stats
    parser.add_argument("gpua", action='store_true') # all of the above

    # git
    git_group = parser.add_argument_group('Git')
    parser.add_argument("gv", action='store_true') # git version

    # development
    dev_group = parser.add_argument_group('development')
    parser.add_argument("dev?", action='store_true') # development langs installed? python, go, nodejs, rust, c++ compiler, c#, etc.
    parser.add_argument("pyv", action='store_true') # python version
    parser.add_argument("gov", action='store_true') # go version
    parser.add_argument("nodev", action='store_true') # nodejs version
    parser.add_argument("rustv", action='store_true') # rust version

    dev install - install rust, nodejs, python, go, etc. ????

    return parser