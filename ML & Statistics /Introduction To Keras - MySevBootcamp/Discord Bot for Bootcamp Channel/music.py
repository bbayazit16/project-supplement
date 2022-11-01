import discord
from discord.ext import commands
import youtube_dl


class music(commands.Cog):
    def __init__(self, client):
        self.client = client

    @commands.command()
    async def join(self, ctx):
        if ctx.author.voice is None:
            await ctx.send("Lütfen önce bir ses kanalına bağlan.")
        voice_channel = ctx.author.voice.channel
        if ctx.voice_client is None:
            await voice_channel.connect()
        else:
            await ctx.voice_client.move_to(voice_channel)

    @commands.command()
    async def disconnect(self, ctx):
        await ctx.voice_client.disconnect()
        await ctx.send("Görüşürüz! :wave:")

    @commands.command()
    async def play(self, ctx, url):
        while True:
            try:
                ctx.voice_client.stop()
                FFMPEG_OPTIONS = {
                    'before_options': '-reconnect 1 -reconnect_streamed 1 -reconnect_delay_max 5', 'options': '-vn'}
                YDL_OPTIONS = {'format': 'bestaudio', 'ignoreerrors': True}
                vc = ctx.voice_client
                await ctx.send("Müziği indiriyorum :arrow_down:")

                with youtube_dl.YoutubeDL(YDL_OPTIONS) as ydl:
                    info = ydl.extract_info(url, download=False)
                    try:
                        url2 = info['formats'][0]['url']
                    except:
                        await ctx.send("Lütfen bir Youtube linki gir.")
                        break
                    source = await discord.FFmpegOpusAudio.from_probe(url2, **FFMPEG_OPTIONS)
                    vc.play(source)
                    break

            except:
                if ctx.author.voice is None:
                    await ctx.send("Lütfen önce bir ses kanalına bağlan.")
                    break
                elif ctx.voice_client is None:
                    voice_channel = ctx.author.voice.channel
                    await voice_channel.connect()
                    continue
                else:
                    voice_channel = ctx.author.voice.channel
                    await ctx.voice_client.move_to(voice_channel)
                    continue

    @commands.command()
    async def pause(self, ctx):
        ctx.voice_client.pause()
        await ctx.send('Müziği durdurdum :pause_button:')

    @commands.command()
    async def resume(self, ctx):
        ctx.voice_client.resume()
        await ctx.send('Müziği kaldığı yerden başlattım :arrow_forward:')


def setup(client):
    client.add_cog(music(client))
