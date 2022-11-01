
# MySevBoootcamp Discord Bot

2021 mySEVAiBootcamp'de kullanılan discord botunun kaynak kodu.  

## İndirme

1. [Discord Developers](https://discord.com/developers) adresine giderek kendinize yeni bir uygulama oluşturun. Uygulamanın token'ını config.py dosyasına giderek yapıştırın.

2. Discord botunu discord developers portaldan admin iznini vererek sunucunuza davet edin. 

3. config.py dosyasına giderek chanel id'leri değiştirin. Eğer help channel eklemek isterseniz, main.py'de ekleyin. (var olanları kopyalayın ve adını değiştirin)

4. Kaynak kodunu indirip repl.it'ye yükleyin. Dikkat, repl.it'ler herkese açık olduğu için token'ı OS kullanarak (Repl.it 'de secrets) saklamanız gerekli.

5. Eğer repl.it kullanıcaksınız botu uptime robot'a bağlayın.

#


Eğer repl.it kullanmak istemiyorsanız, kodu bilgisayırınıza indirip paketeleri yükleyip başlatın.

Eğer macte çalıştırıyorsanız pip3, eğer windowsdaysanız pip kullanın.

```bash
pip3 install -r requirements.txt
```

## Neden Repl.it ve Flask?

Repl.it ve flask beraber hosting görevi görüyor. Uptime robot'a bağlayarak flask sitesini ve bunun yanında botun siteyi kapatsanız da sürekli açık kalmasını sağlıyorsunuz.

## Not
Bazı özellikler aceleyle yazılmıştır, geliştirmek mümkündür. Ancak bu Discord botunda hız pek önemli değil, çalışması yeterli :)

## License
[WTFPL](http://www.wtfpl.net/about/)
