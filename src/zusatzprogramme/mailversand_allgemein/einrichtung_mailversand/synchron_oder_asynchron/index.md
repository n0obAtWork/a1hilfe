# Synchron oder Asynchron

<!-- source: https://amic.de/hilfe/_mailversandsynchronoderasynchron.htm -->

• **Synchron**

Wenn der Datenbankserver selbst E-Mails versenden darf, muss der Steuerparameter 1019 – Mailversand per auf „Datenbank“ stehen. Die E-Mails gehen dann in dem Moment zum Mailserver, wenn die Anforderung dazu erstellt wird.

• **Asynchron mit Dienst**

Nicht immer ist der Datenbankserver mit dem Internet verbunden und kann somit synchron selbst E-Mails versenden.

In diesem Fall ist der Steuerparameter 1019 – Mailversand per auf „Dienst/Exe“ einzustellen. Ein Dienst muss nun auf einem Rechner installiert werden, der Zugang zum Internet hat und zugleich den Datenbankserver erreichen kann. Dieser Dienst startet aus dem A.eins-Unterverzeichnis „Bin64“ die Anwendung „A.eins.MailSvc.exe“ und übergibt folgende Parameter:

• Connectionstring z.B. eng=myengine;dbn=mydbname:links=tcpip;

• Anzahl an Minuten, die zwischen zwei Sendezyklen liegen (default ist 5 min)

• **Asynchron mit Exe**

Ist die Einrichtung eines Dienstes nicht möglich, oder soll testweise ein Mailversand mit Hilfe einer Exe-Datei versendet werden, so kann das Programm „A.eins.Mailer.exe“ aus dem A.eins-Unterverzeichnis „Bin64“ aufgerufen werden. Die Parameter sind dabei die gleichen wie beim Dienst.

• Connectionstring z.B. eng=myengine;dbn=mydbname:links=tcpip;

• Anzahl an Minuten, die zwischen zwei Sendezyklen liegen (default ist 5 min)

<p class="siehe-auch">Siehe auch:</p>

- [Service A.eins Mailversand](./service_a_eins_mailversand.md)
