#! /usr/bin/python3
import discord
import asyncio

friendName = "Old Friend"
elevatorName = "Elevator"
elevator = None

client = discord.Client()

discord_colors = discord.Color.__dict__
colors = list(filter(lambda x: isinstance(discord_colors[x],classmethod), discord_colors))
colors.sort()

async def handle_color(message):    
    words = message.content.split(' ')
    if len(words) == 1:
        await client.send_message(message.channel, 'Available colors are:\n' + (', '.join(colors)))
    if len(words) == 2:
        if (words[1] not in colors):
            await client.send_message(message.channel, 'That is not a valid color.')
        else:
            author = message.author

            print('Changing ' + author.name + '\'s color to ' + words[1])
            
            old_roles = list(filter(lambda r: r.name not in colors, author.roles))
            color_role = list(filter(lambda r: r.name == words[1], message.server.roles))[0]
            new_roles = old_roles + [color_role]
            
            await client.replace_roles(author, *new_roles)
            await client.send_message(message.channel, "Changed your color to " + words[1])

@client.async_event
async def on_server_role_update(before, after):
    if (after.is_everyone):
        await refresh_roles(after.server)
            
async def refresh_roles(server):
    print("Refreshing roles for " + server.name)
    
    perms = discord.Permissions(server.default_role.permissions.value)
    perms.read_message_history = True
    to_edit = list(filter(lambda x: x.permissions != perms, filter(lambda x: x.name in colors or x.name == friendName, server.roles)))
    
    for edit in to_edit:
        await client.edit_role(server, edit, permissions=perms)
        print("Edited " + edit.name)

    perms.read_message_history = False
    if (server.default_role.permissions != perms):
        await client.edit_role(server, server.default_role, permissions=perms)
        print("Edited @everyone")
        
async def create_roles(server):
    print("Creating roles for " + server.name)
    
    role_names = list(map(lambda r: r.name, server.roles))
    to_create = list(filter(lambda x: x not in role_names, colors))
    
    for color in to_create:
        await client.create_role(server, name=color, color=getattr(discord.Color, color)())
        print("Created " + color)
        
def is_admin(member):
    return "Admin" in map(lambda r: r.name, member.roles)

def is_friend(member):
    return friendName in map(lambda r: r.name, member.roles)

@client.async_event
async def on_message(message):
    if (is_admin(message.author) or is_friend(message.author)):
        words = message.content.split(' ')
        command = words[0]
        
        if command == '!help':
            await client.send_message(message.channel, "Available commands:\n!help, !color")
            if is_admin(message.author):
                await client.send_message(message.channel, "Admin-only commands:\n")
                
        if command == '!color':
            await handle_color(message)
                    
        if command =='!debug':
            if message.author.id == '123301224022933504':
                try:
                    to_eval = ' '.join(words[1:])
                    print("Evaling `" + to_eval + "`")
                    result = eval(to_eval)
                except Exception as e:
                    result = '{0.__name__}: {1}'.format(type(e), e)
                await client.send_message(message.channel, "```" + str(result) + "```")

async def create_elevator(client, server):
    global elevator
    voice_channel = discord.utils.get(server.channels, name=elevatorName)
    voice_client = await client.join_voice_channel(voice_channel)
    elevator = await voice_client.create_ytdl_player("https://www.youtube.com/watch?v=VBlFHuCzPgY")
    elevator.volume = .3
    elevator.start()
                        
@client.async_event
async def on_ready():
    print('Logged in!')

    server = list(client.servers)[0]
    await create_roles(server)
    await refresh_roles(server)
    await create_elevator(client, server)
    
    print('Ready')


with open("creds") as f:
    creds = f.readlines()[0].strip()
client.run(creds)
