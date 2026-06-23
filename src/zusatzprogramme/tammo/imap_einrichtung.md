# IMAP - Einrichtung

<!-- source: https://amic.de/hilfe/_tammo_IMAP_EINR.htm -->

Folgende Steuerparameter Einstellungen müssen für das Modul IMAP eingerichtet werden.

| Text | Wert | Beschreibung |
| --- | --- | --- |
| Mailplugin | IMAP | |
| Benutzername / E-Mail | | Die E-Mailadresse oder der Benutzername beim entsprechenden Provider. |
| Passwort | | Hier muss das Passwort des Benutzers eingetragen werden. |
| Domain / Host | | Der Hostname für den IMAP-Abruf beim Provider. |
| Port | | Die Nummer des Ports. |
| SSL verwenden | | Soll kein SSL verwendet werden, kann hier der Wert „FALSE“ eingetragen werden. |

<p class="just-emphasize">Beispiel GMX</p>

Hier eine beispielhafte Einrichtung beim Anbieter GMX.de

| Text | Wert |
| --- | --- |
| Mailplugin | IMAP |
| Benutzername / E-Mail | [example@gmx.de](mailto:example@gmx.de) |
| Passwort | \*\*\*\*\* |
| Domain / Host | imap.gmx.net |
| Port | 993 |
| SSL verwenden | TRUE |

<p class="just-emphasize">Beispiel GMAIL</p>

Hier eine beispielhafte Einrichtung beim Anbieter GMAIL.com

| Text | Wert |
| --- | --- |
| Mailplugin | IMAP |
| Benutzername / E-Mail | [example@gmail.de](mailto:example@gmail.de) |
| Passwort | \*\*\*\*\* |
| Domain / Host | imap.gmail.com |
| Port | 993 |
| SSL verwenden | TRUE |

Für GMAIL sind eventuell noch folgende Kontoeinstellungen nötig.

| Einstellung | Wert |
| --- | --- |
| IMAP-Zugriff | *Gmail -> Einstellungen -> Weiterleitung und POP/IMAP*<br>Der Haken muss bei „IMAP aktivieren“ gesetzt sein. |
| Weniger sichere Apps zulassen | *Mein Konto -> Verbundene Apps und Webseiten*<br>Dort muss die Option „Weniger sichere Apps zulassen“ auf „An“ stehen. |

<p class="just-emphasize">Beispiel KERIO</p>

Hier eine beispielhafte Einrichtung für einen KERIO Server

| Text | Wert |
| --- | --- |
| Mailplugin | IMAP |
| Benutzername / E-Mail | [test@amic.de](mailto:test@amic.de) |
| Passwort | \*\*\*\*\* |
| Domain / Host | 192.168.241.33 |
| Port | 143 |
| SSL verwenden | FALSE |
