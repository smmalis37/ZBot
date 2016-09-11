#! /usr/bin/python3
import discord
import asyncio


friendName = "Old Friend"


client = discord.Client()


discord_colors = discord.Color.__dict__
colors = list(filter(lambda x: isinstance(discord_colors[x],classmethod), discord_colors))
colors.sort()


@asyncio.coroutine
def handle_color(message):    
    words = message.content.split(' ')
    if len(words) == 1:
        yield from client.send_message(message.channel, 'Available colors are:\n' + (', '.join(colors)))
    if len(words) == 2:
        if (words[1] not in colors):
            yield from client.send_message(message.channel, 'That is not a valid color.')
        else:
            author = message.author

            print('Changing ' + author.name + '\'s color to ' + words[1])
            
            old_roles = list(filter(lambda r: r.name not in colors, author.roles))
            color_role = list(filter(lambda r: r.name == words[1], message.server.roles))[0]
            new_roles = old_roles + [color_role]
            
            yield from client.replace_roles(author, *new_roles)
            yield from client.send_message(message.channel, "Changed your color to " + words[1])

@client.async_event
def on_server_role_update(before, after):
    if (after.is_everyone):
        yield from refresh_roles(after.server)
            
@asyncio.coroutine
def refresh_roles(server):
    print("Refreshing roles for " + server.name)
    
    perms = discord.Permissions(server.default_role.permissions.value)
    perms.read_message_history = True
    to_edit = list(filter(lambda x: x.permissions != perms, filter(lambda x: x.name in colors or x.name == friendName, server.roles)))
    
    for edit in to_edit:
        yield from client.edit_role(server, edit, permissions=perms)
        print("Edited " + edit.name)

    perms.read_message_history = False
    if (server.default_role.permissions != perms):
        yield from client.edit_role(server, server.default_role, permissions=perms)
        print("Edited @everyone")
        
@asyncio.coroutine
def create_roles(server):
    print("Creating roles for " + server.name)
    
    role_names = list(map(lambda r: r.name, server.roles))
    to_create = list(filter(lambda x: x not in role_names, colors))
    
    for color in to_create:
        yield from client.create_role(server, name=color, color=getattr(discord.Color, color)())
        print("Created " + color)
        
def is_admin(member):
    return "Admin" in map(lambda r: r.name, member.roles)

def is_friend(member):
    return friendName in map(lambda r: r.name, member.roles)

@client.async_event
def on_message(message):
    if (is_admin(message.author) or is_friend(message.author)):
        words = message.content.split(' ')
        command = words[0]
        
        if command == '!help':
            yield from client.send_message(message.channel, "Available commands:\n!help, !color")
            if is_admin(message.author):
                yield from client.send_message(message.channel, "Admin-only commands:\n")
                
        if command == '!color':
            yield from handle_color(message)
                    
        if command =='!debug':
            if message.author.id == '123301224022933504':
                try:
                    command = ' '.join(words[1:])
                    print("Evaling " + command)
                    result = eval(command)
                except Exception as e:
                    result = '{0.__name__}: {1}'.format(type(e), e)
                    yield from client.send_message(message.channel, "'''" + result + "'''")
            
@client.async_event
def on_ready():
    print('Logged in!')

    for server in client.servers:
        yield from create_roles(server)
        yield from refresh_roles(server)
        
    print('Ready')


with open("creds") as f:
    creds = f.readlines()[0].strip()
client.run(creds)
