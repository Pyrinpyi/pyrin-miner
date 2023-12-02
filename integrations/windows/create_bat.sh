echo REM When mining to a local node, you can drop the -s option. > ${1}/mine.bat
echo echo ============================================================ >> ${1}/mine.bat
echo echo = Running Kaspa Miner with default .bat. Edit to configure = >> ${1}/mine.bat
echo echo ============================================================ >> ${1}/mine.bat
echo :start >> ${1}/mine.bat
echo ${1}.exe -a qrzs2hd6rtcx2zd4dzkzrpqjx4jg8ndmqqjle8j9cpp93gg059tludxxvvfqd -s n.seeder1.kaspad.net >> ${1}/mine.bat
echo goto start >> ${1}/mine.bat


# target\release\pyrin-miner -a pyrin:qzn54t6vpasykvudztupcpwn2gelxf8y9p84szksr73me39mzf69uaalnymtx -s localhost
