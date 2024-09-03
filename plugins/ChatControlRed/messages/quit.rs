# ---------------------------------------------------------------------------------
# Quit messages. Uses the same syntax as files in rules/ folder but operators
# are slightly different. For documentation and a quick tutorial, see:
# https://github.com/kangarko/ChatControl-Red/wiki/messages
#
# Rules in this file are read from top to bottom. Set 'Stop_On_First_Match' 
# key in settings.yml only send the first message we can, to the player (based on
# operator conditions).
# ---------------------------------------------------------------------------------

# An example of custom leave message based on the playername
# ( This is case sensitive )
#group kangarko-leave-message
#require sender script "{player}" == "Kangarko"
#message: 
#- &cKangarko &7jumped until he found the exit

# An example of custom quit message based on a permission.
# Players/ranks with the permission 'group.vip' will use this message instead of the default one
# (Only if 'Stop_On_First_Match' is set to 'true' in the settings file)
#group vip-quit-message
#require sender perm group.vip
#message: 
#- &e[VIP] &c{player} &7left the server

group default
message:
- &7{player} &fhas quit the game
