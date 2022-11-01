import discord
import hashlib
import json  # eval
import os
import random
import requests
import time
from contextlib import redirect_stdout
from discord.ext import commands
from io import StringIO

import music
from config import token, sudo_users_list, help_words_list, help_channel_1_id, help_channel_2_id, help_channel_3_id, \
    help_channel_4_id, helper_role_id, moderator_role_id, profanity_list, mod_channel_id, welcome_channel_id, \
    introduction_channel_id, debug, sudo, count_messages, auto_compile, topcount, filterpf, filterhelp
from flask_server import runner

# from config import *
# repl.it import * izin vermiyor

# r = requests.head(url="https://discord.com/api/v1")
# try:
#     print(f"{int(r.headers['Retry-After']) / 60}")
# except:
#     print("Bloklanmadım")

cogs = [music]


def status_reader(doc):
    with open(doc, 'r') as f:
        lines = f.readlines()
        fstr = [lines.strip() for lines in lines]
        is_occupied = eval(fstr[0])
        try:
            occupant = eval(fstr[1])
        except:
            occupant = fstr[1]
        return is_occupied, occupant


class hc_status:
    def __init__(self, doc):
        self.doc = doc


help_channel_1_status = hc_status('config/hc_1.txt')
help_channel_2_status = hc_status('config/hc_2.txt')
help_channel_3_status = hc_status('config/hc_3.txt')
help_channel_4_status = hc_status('config/hc_4.txt')

help_cd = {
    help_channel_1_id: help_channel_1_status,
    help_channel_2_id: help_channel_2_status,
    help_channel_3_id: help_channel_3_status,
    help_channel_4_id: help_channel_4_status,
}

with open(profanity_list, 'r') as pf:
    profanity_words = [word.strip() for word in pf.readlines()]

with open(help_words_list, 'r') as pf:
    help_words = [word.strip() for word in pf.readlines()]

with open(sudo_users_list, 'r') as pf:
    sudo_users = [word.strip() for word in pf.readlines()]

start = time.time()
intents = discord.Intents.default()
intents.members = True
client = commands.Bot('!', case_insensitive=True, intents=intents)
client.remove_command("help")
help_channel_ids = list(help_cd.keys())

for i in range(len(cogs)):
    cogs[i].setup(client)


def mod(author):
    if str(moderator_role_id) in str(author.roles):
        return True
    if sudo and str(author.id) in sudo_users:
        return True
    return False


@client.event
async def on_ready():
    await client.change_presence(activity=discord.Game('with Neural Networks'))
    print("Bot başlatıldı.")


@client.command('ping')
async def ping(ctx):
    await ctx.send("**Pong**")
    await ctx.send('https://giphy.com/gifs/cat-animal-ping-pong-fvA1ieS8rEV8Y')


@client.command('musichelp', aliases=['music'])
async def musichelp(ctx):
    desc = (
        "!play <youtube linki> yazarak müziği başlatın. !pause müziği durdurur ve !resume müziği kaldığı yerden başlatır. Kanal değiştirmek için başka bir kanala gidip !join yazın. Dinlemeyi bitirdiğiniz zaman ayrılmadan lütfen !disconnect yazın.")
    embedded_message = discord.Embed(
        title="Komutlar:", description=desc, color=0x0b6cdb)
    await ctx.send(embed=embedded_message)


@client.command('mentörler', aliases=['mentör'])
async def mentörler(ctx):
    await ctx.send('Mentörlerimiz:\nEge Demir\nHüseyin Yağız Devre')


@client.command('seepoints', aliases=['seepoint'])
async def seepoints(ctx, *, usrnick):
    try:
        with open(f"points/{usrnick.lower()}", "r") as opekf:
            await ctx.send(f"{usrnick} adlı kişinin puan sayısı: {opekf.read()}")
    except:
        await ctx.send("Bu kişi daha önce hiç konuşmamış!")


@client.command('website', aliases=['Vebsitesi', 'veb', 'web', 'site'])
async def website(ctx):
    await ctx.send('https://sites.google.com/sev.org.tr/mysev-ai-bootcamp/home')


@client.command('format')
async def format(ctx):
    await ctx.send(
        'Kodu Python\'da formatlayarak göndermek için lütfen 3 tane ` karakteri kullanın. https://imgur.com/a/XalEbWE')


@client.command('program', aliases=['schedule'])
async def program(ctx):
    await ctx.send(
        'https://docs.google.com/spreadsheets/d/1Dlg_jkF6UkxNE5ICzEd0H4pWVEr6ktnfiMBRJrMhqk4/edit#gid=201648416')


@client.command('badge', aliases=['badges'])
async def badge(ctx):
    await ctx.send('https://mysevbootcamp.sev.org.tr/badges')


@client.command('help', aliases=['yardım', 'yardim'])
async def help(ctx):
    desc = (
        "!help\n !ping\n !python\n !mentörler\n !website\n !badges\n !format\n !program\n !close\n !github\n !musichelp\n !points\n !top\n !curl\n !sha256\n !modhelp")
    embedded_message = discord.Embed(
        title="Komutlar:", description=desc, color=0x0b6cdb)
    await ctx.send(embed=embedded_message)


@client.command('github')
async def github(ctx):
    await ctx.send("Kaynak kodum: https://github.com/bbayazit16/mySEVBootcampDiscordBot")


@client.command('close', aliases=['kapat', 'kapa', 'end', 'temizle'])
async def close(ctx):
    if ctx.channel.id in help_channel_ids and str(ctx.author) == status_reader(help_cd[ctx.channel.id].doc)[1]:
        with open(help_cd[ctx.channel.id].doc, 'w') as f:
            f.write(f"False\n")
            f.write(f"None")
        pins = await ctx.channel.pins()
        await pins[0].unpin()
        desc = "Buraya sorunu yazarak yardım almaya başlayabilirsin."
        embedded_message = discord.Embed(
            title="Boş Yardım Kanalı", description=desc, color=0x0b6cdb)
        embedded_message.add_field(
            name="Lütfen:",
            value="•Sorunuzu göndermeden doğru dille formatlamayı unutmayın. Daha fazla bilgi için başka bir kanalda !format yaz.\n"
                  "•Sorun cevaplandıktan sonra !close yazarak yardımı bitirmeyi unutma lütfen.\n"
                  "•Soru sormaktan çekinmeyin :)")
        await ctx.send(embed=embedded_message)


@client.command('modhelp', aliases=['modyardım', 'modyardim', 'moderatöryardım'])
async def modhelp(ctx):
    if mod(ctx.message.author):
        desc = (
            "!status\n !runtime")
        embedded_message = discord.Embed(
            title="Moderatör Komutları:", description=desc, color=0x0b6cdb)
        await ctx.message.channel.send(embed=embedded_message)
    else:
        await ctx.send("Bu komutu sadece moderatörler kullanabilir.")


@client.command('curl')
async def curl(ctx, urlstrm, *keys):
    try:
        request = requests.get(urlstrm).text
        if keys:
            directory = ""
            for arg in keys:
                if arg.isdigit():
                    directory += f'[{arg}]'
                else:
                    directory += f'["{arg}"]'
            execmap = f"json.loads(request){directory}"
            vals = eval(execmap)
            await ctx.send(vals)
        else:
            await ctx.send(request)
    except Exception as e:
        await ctx.send(str(e))


@client.command('hash', aliases=['sha256', 'sha'])
async def config(ctx, *, tbh):
    try:
        await ctx.send(f"{hashlib.sha256(str(tbh).encode('utf-8')).hexdigest()}")
    except Exception as e:
        await ctx.send(str(e))


@client.command('top', aliases=['ranking', 'ranks', 'sıralama', 'siralama'])
# her kullanıcı için ayrı txt olmasının sebebi daha sonradan değiştirme ihtimalimin olması
# sonradan tek tek değiştirmek istemem
async def top(ctx, count=topcount):
    usr_points = []
    usr_names = []
    for file in os.listdir('points'):
        with open(f'points/{file}', 'r') as f:
            point = int(f.readline())
            usr_points.append(point)
            usr_names.append(str(file))

    zips = zip(usr_points, usr_names)
    sortzip = sorted(zips, reverse=True)
    tuples = zip(*sortzip)
    usr_points, usr_names = [list(tuple) for tuple in tuples]
    desc = ""
    try:
        finalcount = topcount if count == topcount else count
        for i in range(finalcount):
            desc += f"{usr_names[i]}: {usr_points[i]}\n"
        embedded_message_x = discord.Embed(
            title=f"En Yüksek Puanı Bulunan {finalcount} Kişi:", description=desc, color=0x0b6cdb)
        await ctx.send(embed=embedded_message_x)
    except:
        await ctx.send("Bu kadar kullanıcımız yok gibi gözüküyor.")


@client.command('run', aliases=['python3', 'python'])
async def run(ctx):
    def check_run(msg):
        return msg.author == ctx.author

    await ctx.send("Formatlayarak çalıştırmak istediğin python kodunu gir:")
    pyc_m = await client.wait_for("message", check=check_run)
    pyc_c = pyc_m.content
    pyc = pyc_c.replace('```Python', "").replace('```', '')
    filename = str(random.randint(2 ** 16, 2 ** 32))
    # scriptler sonradan erişim gerekebilir diye silinmiyor.
    with open(f'scriptlogs/{filename}', 'w+') as tbrc:
        tbrc.write(pyc)
    with open(f'scriptlogs/{filename}', 'a') as tblog:
        tblog.write("\n")
        tblog.write(f"# Submitted by {ctx.author}")
    try:
        strio = StringIO()
        with redirect_stdout(strio):
            exec(open(f"scriptlogs/{filename}").read())
        output = strio.getvalue()
        await ctx.send(output)
    except Exception as e:
        await ctx.send(str(e))


@client.command('status')
async def status(ctx):
    if mod(ctx.message.author):
        await ctx.send("Playing, watching ya da listening üçlüsünden birini seçin:")

        def activity_check(msg):
            return msg.author == ctx.author and msg.channel == ctx.channel and msg.content.lower() in ["playing",
                                                                                                       "watching",
                                                                                                       "listening"]

        activity = await client.wait_for("message", check=activity_check)
        await ctx.send("Lütfen bota status yazın:")

        def status_check(msg):
            return msg.author == ctx.author and msg.channel == ctx.channel

        status = await client.wait_for("message", check=status_check)
        await ctx.send(f"Yeni status: {activity.content.capitalize()} {status.content}. Devam edeyim mi? Evet/Hayır?")

        def final_check(msg):
            return msg.author == ctx.author and msg.channel == ctx.channel and msg.content.lower() in ["evet", "yes",
                                                                                                       "y", "hayır",
                                                                                                       "no", "n",
                                                                                                       "iptal"]

        final_checkmark = await client.wait_for("message", check=final_check)

        if final_checkmark.content.lower() in ["evet", "yes", "y"]:
            if activity.content.lower() == "playing":
                await client.change_presence(activity=discord.Game(name=status.content))
            if activity.content.lower() == "watching":
                await client.change_presence(
                    activity=discord.Activity(type=discord.ActivityType.watching, name=status.content))
            if activity.content.lower() == "listening":
                await client.change_presence(
                    activity=discord.Activity(type=discord.ActivityType.listening, name=status.content))
        else:
            return
    else:
        await ctx.send("Bu komutu sadece moderatörler kullanabilir.")


@client.command('runtime', aliases=['uptime', 'süre', 'zaman'])
async def uptime(ctx):
    if mod(ctx.message.author):
        runtime_secs = round(time.time() - start, 2)
        await ctx.send(f"{round(runtime_secs / 3600, 2)} saattir aralıksız hizmetinizdeyim.")
    else:
        await ctx.send("Bu komutu sadece moderatörler kullanabilir.")


@client.event
async def on_message(message):
    if message.author == client.user:
        return

    if count_messages:

        if message.author.nick:
            author = message.author.nick.lower()
        else:
            author = message.author.name.lower()

        try:
            with open(f'points/{author}', 'r') as f:
                pass
        except:
            with open(f'points/{author}', 'w+') as f:
                f.write('0')
        with open(f'points/{author}', 'r') as f:
            user_point = int(f.readline())
            user_point += 1
        with open(f'points/{author}', 'w') as f:
            f.write(str(user_point))

    if any(word in message.content.lower() for word in help_words) and filterhelp:
        if message.channel.id not in help_channel_ids and message.content.lower()[:1] != '!':
            await message.channel.send(
                f'{message.author.mention} Yardım almak için lütfen yardım kanallarından birini ya da !help komutunu kullan.')

    if any(word in message.content.lower() for word in profanity_words) and filterpf:
        await message.delete()
        await message.channel.send(f'{message.author.mention} lütfen sözlerine dikkat et!')
        mod_channel = client.get_channel(mod_channel_id)
        await mod_channel.send(f'{message.author.mention} {message.channel.mention} içinde "{message.content}" dedi.')

    if message.channel.id in help_channel_ids and not status_reader(help_cd[message.channel.id].doc)[0]:
        await message.pin()
        with open(help_cd[message.channel.id].doc, 'w') as f:
            f.write(f"True\n")
            f.write(f"{message.author}")
        await message.channel.send(f"<@&{helper_role_id}>!!!!")

    if debug:
        print("Mesaj Bilgileri:")
        print(message.content.lower(), message.author,
              message.channel, message.channel.id)
        try:
            print("Yardım Kanalı Bilgileri:")
            print(status_reader(help_cd[message.channel.id].doc)[0])
            print(status_reader(help_cd[message.channel.id].doc)[1])
        except:
            pass

    if message.content.startswith("!points") or message.content.startswith("!point"):
        if message.author.nick:
            author = message.author.nick.lower()
        else:
            author = message.author.name.lower()
        with open(f'points/{author}', 'r') as f:
            user_point_rq = int(f.readline())
        await message.channel.send(f"Senin puanların: {user_point_rq}")

    if message.content.startswith("```Python") and auto_compile:
        pyc_c = message.content
        #  pyc = pyc_c.replace('```Python', "").replace('```', '')
        pyc = pyc_c.replace('Python', '').split('```')[1]
        filename = str(random.randint(2 ** 16, 2 ** 32))
        # scriptler sonradan erişim gerekebilir diye silinmiyor.
        with open(f'scriptlogs/{filename}', 'w+') as tbrc:
            tbrc.write(pyc)
        with open(f'scriptlogs/{filename}', 'a') as tblog:
            tblog.write("\n")
            tblog.write(
                f"# {message.author} tarafından {time.time()} zamanında gönderildi. ")
        try:
            strio = StringIO()
            with redirect_stdout(strio):
                exec(open(f"scriptlogs/{filename}").read()[1:])
            output = strio.getvalue()
            await message.channel.send(output)
        except Exception as e:
            await message.channel.send(str(e))

    if message.content.startswith('bad bot'):
        await message.channel.send(":pleading_face: :sob:")

    if message.content.startswith('good bot'):
        await message.channel.send(":smile: :blush:")

    await client.process_commands(message)


@client.event
async def on_member_join(member):
    await client.get_channel(welcome_channel_id).send(
        f"Hoşgeldin, {member.mention}! Lütfen kendini <#{introduction_channel_id}> kanalında tanıt. Lütfen /nick yazarak ismini gerçek isminle değiştir.")


runner()
client.run(token)
