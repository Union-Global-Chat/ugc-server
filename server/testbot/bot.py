import discord
from websockets.asyncio.client import connect
import dotenv
import aiohttp

import os
import logging
import json


dotenv.load_dotenv()

logger = logging.getLogger()

client = discord.Client(intents=discord.Intents.all())

ugc_token = os.getenv("UGC_TOKEN")


@client.event
async def on_ready() -> None:
    logger.info("Now booted")
    global ws
    ws = await connect("ws://localhost:3000/gateway")
    print(await ws.recv())
    await ws.send(json.dumps({
        "t": "Identify",
        "d": {
            "token": ugc_token,
        }
    }))
    while True:
        print(await ws.recv())


@client.event
async def on_message(message) -> None:
    async with aiohttp.ClientSession() as session:
        async with session.post("http://localhost:3000/messages", json={
            "channel": {
                "name": message.channel.name,
                "id": str(message.channel.id)
            },
            "author": {
                "username": message.author.name,
                "discriminator": message.author.discriminator,
                "id": str(message.author.id),
                "avatarURL": message.author.avatar.url,
                "bot": message.author.bot,
            },
            "guild": {
                "name": message.guild.name,
                "id": str(message.guild.id),
                "iconURL": message.guild.icon.url,
            },
            "message": {
                "content": message.content,
                "id": str(message.id),
                "clean_content": message.clean_content
            }
        }, headers={
            "Authorization": "Bearer {}".format(ugc_token)
        }) as res:
            pass


client.run(os.getenv("DISCORD_TOKEN"))