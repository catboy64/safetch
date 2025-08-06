//const BLACK: &str = "\x1b[1;30m";
const RED: &str = "\x1b[1;31m";
const GREEN: &str = "\x1b[1;32m";
//const YELLOW: &str = "\x1b[1;33m";
const BLUE: &str = "\x1b[1;34m";
const PURPLE: &str = "\x1b[1;35m";
const CYAN: &str = "\x1b[1;36m";
const WHITE: &str = "\x1b[1;37m";
const COLOR_END: &str = "\x1b[0m";


pub fn get_ascii(os_id: &str) -> String {
    // returns the appropriate ascii, acording to os_name, might need to replace os_name by id
    match os_id {
        "arch"    => return format!("                                       
                   {CYAN}-`                  {COLOR_END}
                  {CYAN}.o+`                 {COLOR_END}
                 {CYAN}`ooo/                 {COLOR_END}
                {CYAN}`+oooo:                {COLOR_END}
               {CYAN}`+oooooo:               {COLOR_END}
               {CYAN}-+oooooo+:              {COLOR_END}
             {CYAN}`/:-:++oooo+:             {COLOR_END}
            {CYAN}`/++++/+++++++:            {COLOR_END}
           {CYAN}`/++++++++++++++:           {COLOR_END}
          {CYAN}`/+++ooooooooooooo/`         {COLOR_END}
         {CYAN}./ooosssso++osssssso+`        {COLOR_END}
        {CYAN}.oossssso-````/ossssss+`       {COLOR_END}
       {CYAN}-osssssso.      :ssssssso.      {COLOR_END}
      {CYAN}:osssssss/        osssso+++.     {COLOR_END}
     {CYAN}/ossssssss/        +ssssooo/-     {COLOR_END}
   {CYAN}`/ossssso+/:-        -:/+osssso+-   {COLOR_END}
  {CYAN}`+sso+:-`                 `.-/+oso:  {COLOR_END}
 {CYAN}`++:.                           `-/+/ {COLOR_END}
 {CYAN}.`                                 `/ {COLOR_END}"),
        
        "debian" => return format!("
       {WHITE}_,met$$$$$gg.             {COLOR_END}
    {WHITE},g$$$$$$$$$$$$$$$P.          {COLOR_END}
  {WHITE},g$$P\"        \"\"\"Y$$.\".        {COLOR_END}
 {WHITE},$$P'              `$$$.        {COLOR_END}
{WHITE}',$$P       ,ggs.     `$$b:      {COLOR_END}
{WHITE}`d$$'     ,$P\"'   {RED}.{WHITE}    $$$       {COLOR_END}
 {WHITE}$$P      d$'     {RED},{WHITE}    $$P       {COLOR_END}
 {WHITE}$$:      $$.   {RED}-{WHITE}    ,d$$'       {COLOR_END}
 {WHITE}$$;      Y$b._   _,d$P'         {COLOR_END}
 {WHITE}Y$$.    {RED}`.{WHITE}`\"Y$$$$P\"'            {COLOR_END}
{WHITE} `$$b      {RED}\"-.__                 {COLOR_END}
{WHITE}  `Y$$                           {COLOR_END}
   {WHITE}`Y$$.                         {COLOR_END}
     {WHITE}`$$b.                       {COLOR_END}
       {WHITE}`Y$$b.{COLOR_END}
          {WHITE}`\"Y$b._{COLOR_END}
              {WHITE}`\"\"\""),



        "linuxmint"   => return format!("                                          
{WHITE}             ...-:::::-...                {COLOR_END}
{WHITE}          .-MMMMMMMMMMMMMMM-.             {COLOR_END}
{WHITE}      .-MMMM{GREEN}`..-:::::::-..`{WHITE}MMMM-.       {COLOR_END}  
{WHITE}    .:MMMM{GREEN}.:MMMMMMMMMMMMMMM:.{WHITE}MMMM:.       {COLOR_END}
{WHITE}   -MMM{GREEN}-M---MMMMMMMMMMMMMMMMMMM.{WHITE}MMM-      {COLOR_END}
{WHITE} `:MMM{GREEN}:MM`  :MMMM:....::-...-MMMM:{WHITE}MMM:`   {COLOR_END}
{WHITE} :MMM{GREEN}:MMM`  :MM:`  ``    ``  `:MMM:{WHITE}MMM:   {COLOR_END}
{WHITE}.MMM{GREEN}.MMMM`  :MM.  -MM.  .MM-  `MMMM.{WHITE}MMM.  {COLOR_END}
{WHITE}:MMM{GREEN}:MMMM`  :MM.  -MM-  .MM:  `MMMM-{WHITE}MMM:  {COLOR_END}
{WHITE}:MMM{GREEN}:MMMM`  :MM.  -MM-  .MM:  `MMMM:{WHITE}MMM:  {COLOR_END}
{WHITE}:MMM{GREEN}:MMMM`  :MM.  -MM-  .MM:  `MMMM-{WHITE}MMM:  {COLOR_END}
{WHITE}.MMM{GREEN}.MMMM`  :MM:--:MM:--:MM:  `MMMM.{WHITE}MMM.  {COLOR_END}
{WHITE} :MMM{GREEN}:MMM-  `-MMMMMMMMMMMM-`  -MMM-{WHITE}MMM:   {COLOR_END}
{WHITE}  :MMM{GREEN}:MMM:`                `:MMM:{WHITE}MMM:    {COLOR_END}
{WHITE}   .MMM{GREEN}.MMMM:--------------:MMMM.{WHITE}MMM.     {COLOR_END}
{WHITE}     '-MMMM{GREEN}.-MMMMMMMMMMMMMMM-.{WHITE}MMMM-'      {COLOR_END}
{WHITE}       '.-MMMM{GREEN}``--:::::--``{WHITE}MMMM-.'        {COLOR_END}
{WHITE}            '-MMMMMMMMMMMMM-'               {COLOR_END}
{WHITE}               ``-:::::-``                  {COLOR_END}"),
        "kali"    => return format!("
{BLUE}..............                                      {COLOR_END}
{BLUE}            ..,;:ccc,.                              {COLOR_END}
{BLUE}          ......''';lxO.                            {COLOR_END}
{BLUE}.....''''..........,:ld;                            {COLOR_END}
{BLUE}           .';;;:::;,,.x,                           {COLOR_END}
{BLUE}      ..'''.            0Xxoc:,.  ...               {COLOR_END}
{BLUE}  ....                ,ONkc;,;cokOdc',.             {COLOR_END}
{BLUE} .                   OMo           ':{RED}dd{BLUE}o.           {COLOR_END}
{BLUE}                    dMc               :OO;          {COLOR_END}
{BLUE}                    0M.                 .:o.        {COLOR_END}
{BLUE}                    ;Wd                             {COLOR_END}
{BLUE}                     ;XO,                           {COLOR_END}
{BLUE}                       ,d0Odlc;,..                  {COLOR_END}
{BLUE}                           ..',;:cdOOd::,.          {COLOR_END}
{BLUE}                                    .:d;.':;.       {COLOR_END}
{BLUE}                                       'd,  .'      {COLOR_END}
{BLUE}                                         ;l   ..    {COLOR_END}
{BLUE}                                          .o        {COLOR_END}
{BLUE}                                            c       {COLOR_END}
{BLUE}                                            .'      {COLOR_END}
{BLUE}                                             .      {COLOR_END}"),

        "fedora"  => return format!("
{BLUE}             .',;::::;,'.                 {COLOR_END}
{BLUE}         .';:cccccccccccc:;,.             {COLOR_END}
{BLUE}      .;cccccccccccccccccccccc;.          {COLOR_END}
{BLUE}    .:cccccccccccccccccccccccccc:.        {COLOR_END}
{BLUE}  .;ccccccccccccc;{WHITE}.:dddl:.{BLUE};ccccccc;.      {COLOR_END}
{BLUE} .:ccccccccccccc;{WHITE}OWMKOOXMWd{BLUE};ccccccc:.     {COLOR_END}
{BLUE}.:ccccccccccccc;{WHITE}KMMc{BLUE};cc;{WHITE}xMMc{BLUE};ccccccc:.    {COLOR_END}
{BLUE},cccccccccccccc;{WHITE}MMM.{BLUE};cc;{WHITE};WW:{BLUE};cccccccc,    {COLOR_END}
{BLUE}:cccccccccccccc;{WHITE}MMM.{BLUE};cccccccccccccccc:    {COLOR_END}
{BLUE}:ccccccc;{WHITE}oxOOOo{BLUE};{WHITE}MMM0OOk.{BLUE};cccccccccccc:    {COLOR_END}
{BLUE}cccccc;{WHITE}0MMKxdd:{BLUE};{WHITE}MMMkddc.{BLUE};cccccccccccc;    {COLOR_END}
{BLUE}ccccc;{WHITE}XM0'{BLUE};cccc;{WHITE}MMM.{BLUE};cccccccccccccccc'    {COLOR_END}
{BLUE}ccccc;{WHITE}MMo{BLUE};ccccc;{WHITE}MMW.{BLUE};ccccccccccccccc;     {COLOR_END}
{BLUE}ccccc;{WHITE}0MNc.{BLUE}ccc{WHITE}.xMMd{BLUE};ccccccccccccccc;      {COLOR_END}
{BLUE}cccccc;{WHITE}dNMWXXXWM0:{BLUE};cccccccccccccc:,       {COLOR_END}
{BLUE}cccccccc;{WHITE}.:odl:.{BLUE};cccccccccccccc:,.{COLOR_END}
{BLUE}:cccccccccccccccccccccccccccc:'.{COLOR_END}
{BLUE}.:cccccccccccccccccccccc:;,..{COLOR_END}
{BLUE}  '::cccccccccccccc::;,.{COLOR_END}"),

      "manjaro" => return format!("
{GREEN}██████████████████  ████████   {COLOR_END}
{GREEN}██████████████████  ████████   {COLOR_END}
{GREEN}██████████████████  ████████   {COLOR_END}
{GREEN}██████████████████  ████████   {COLOR_END}
{GREEN}████████            ████████   {COLOR_END}
{GREEN}████████  ████████  ████████   {COLOR_END}
{GREEN}████████  ████████  ████████   {COLOR_END}
{GREEN}████████  ████████  ████████   {COLOR_END}
{GREEN}████████  ████████  ████████   {COLOR_END}
{GREEN}████████  ████████  ████████   {COLOR_END}
{GREEN}████████  ████████  ████████   {COLOR_END}
{GREEN}████████  ████████  ████████   {COLOR_END}
{GREEN}████████  ████████  ████████   {COLOR_END}
{GREEN}████████  ████████  ████████   {COLOR_END}"),

      "ubuntu" => return format!("
{RED}            .-/+oossssoo+\\-.             {COLOR_END}
{RED}        ´:+ssssssssssssssssss+:`         {COLOR_END}
{RED}      -+ssssssssssssssssssyyssss+-       {COLOR_END}
{RED}    .ossssssssssssssssss{WHITE}dMMMNy{RED}sssso.     {COLOR_END}
{RED}   /sssssssssss{WHITE}hdmmNNmmyNMMMMh{RED}ssssss\\    {COLOR_END}
{RED}  +sssssssss{WHITE}hm{RED}yd{WHITE}MMMMMMMNddddy{RED}ssssssss+   {COLOR_END}
{RED} /ssssssss{WHITE}hNMMM{RED}yh{WHITE}hyyyyhmNMMMNh{RED}ssssssss\\  {COLOR_END}
{RED}.ssssssss{WHITE}dMMMNh{RED}ssssssssss{WHITE}hNMMMd{RED}ssssssss. {COLOR_END}
{RED}+ssss{WHITE}hhhyNMMNy{RED}ssssssssssss{WHITE}yNMMMy{RED}sssssss+ {COLOR_END}
{RED}oss{WHITE}yNMMMNyMMh{RED}ssssssssssssss{WHITE}hmmmh{RED}ssssssso {COLOR_END}
{RED}oss{WHITE}yNMMMNyMMh{RED}sssssssssssssshmmmh{RED}ssssssso {COLOR_END}
{RED}+ssss{WHITE}hhhyNMMNy{RED}ssssssssssss{WHITE}yNMMMy{RED}sssssss+ {COLOR_END}
{RED}.ssssssss{WHITE}dMMMNh{RED}ssssssssss{WHITE}hNMMMd{RED}ssssssss. {COLOR_END}
{RED} \\ssssssss{WHITE}hNMMM{RED}yh{WHITE}hyyyyhdNMMMNh{RED}ssssssss/  {COLOR_END}
{RED}  +sssssssss{WHITE}dm{RED}yd{WHITE}MMMMMMMMddddy{RED}ssssssss+  {COLOR_END}
{RED}   \\sssssssssss{WHITE}hdmNNNNmyNMMMMh{RED}ssssss/ {COLOR_END}
{RED}    .ossssssssssssssssss{WHITE}dMMMNy{RED}sssso.{COLOR_END}
{RED}      -+sssssssssssssssss{WHITE}yyy{RED}ssss+-{COLOR_END}
{RED}        `:+ssssssssssssssssss+:`{COLOR_END}
{RED}            .-\\+oossssoo+/-.{COLOR_END}"),

      "slackware" => return format!("
{BLUE}                  :::::::                   {COLOR_END}
{BLUE}            :::::::::::::::::::             {COLOR_END}
{BLUE}         :::::::::::::::::::::::::          {COLOR_END}
{BLUE}       ::::::::{WHITE}cllcccccllllllll{BLUE}::::::       {COLOR_END}
{BLUE}    :::::::::{WHITE}lc               dc{BLUE}:::::::     {COLOR_END}
{BLUE}   ::::::::{WHITE}cl   clllccllll    oc{BLUE}:::::::::   {COLOR_END}
{BLUE}  :::::::::{WHITE}o   lc{BLUE}::::::::{WHITE}co   oc{BLUE}::::::::::  {COLOR_END}
{BLUE} ::::::::::{WHITE}o    cccclc{BLUE}:::::{WHITE}clcc{BLUE}:::::::::::: {COLOR_END}
{BLUE} :::::::::::{WHITE}lc        cclccclc{BLUE}::::::::::::: {COLOR_END}
{BLUE}::::::::::::::{WHITE}lcclcc          lc{BLUE}::::::::::::{COLOR_END}
{BLUE}::::::::::{WHITE}cclcc{BLUE}:::::{WHITE}lccclc     oc{BLUE}:::::::::::{COLOR_END}
{BLUE}::::::::::{WHITE}o    l{BLUE}::::::::::{WHITE}l    lc{BLUE}:::::::::::{COLOR_END}
{BLUE} :::::{WHITE}cll{BLUE}:{WHITE}o     clcllcccll     o{BLUE}::::::::::: {COLOR_END}
{BLUE} :::::{WHITE}occ{BLUE}:{WHITE}o                  clc{BLUE}::::::::::: {COLOR_END}
{BLUE}  ::::{WHITE}ocl{BLUE}:{WHITE}ccslclccclclccclclc{BLUE}:::::::::::::{COLOR_END}
{BLUE}   :::{WHITE}oclcccccccccccccllllllllllllll{BLUE}:::::  {COLOR_END}
{BLUE}    ::{WHITE}lcc1lcccccccccccccccccccccccco{BLUE}:::: {COLOR_END}
{BLUE}      ::::::::::::::::::::::::::::::::    {COLOR_END}
{BLUE}        ::::::::::::::::::::::::::::  {COLOR_END}
{BLUE}           :::::::::::::::::::::: {COLOR_END}
{BLUE}                ::::::::::::{COLOR_END}"),

      "paran" => return format!("{COLOR_END}
{PURPLE}         _      {COLOR_END}
{PURPLE}        /\\ \\    {COLOR_END}
{PURPLE}       /  \\ \\   {COLOR_END}
{PURPLE}      / /\\ \\ \\  {COLOR_END}
{PURPLE}     / / /\\ \\_\\ {COLOR_END}
{PURPLE}    / / /_/ / / {COLOR_END}
{PURPLE}   / / /__\\/ /  {COLOR_END}
{PURPLE}  / / /_____/   {COLOR_END}
{PURPLE} / / /          {COLOR_END}
{PURPLE}/ / /           {COLOR_END}
{PURPLE}\\/_/            {COLOR_END}
                {COLOR_END}
                {COLOR_END}
                {COLOR_END}
"),

        _       => return String::from("distro not supported (yet)"),
    }
}
