# Standardwaagenprofil-Unterstützung: Aeinswiege

<!-- source: https://amic.de/hilfe/standardwaagenprofiluntersttzu.htm -->

AeinsWiege ist ein Programm, welches das Wiegen von solchen Waage-Protokollen unterstützen soll, bei denen die Bordmittel von Aeins nicht ausreichen. 

AeinsWiege ist so konzipiert das es "einfach" mit Hilfe des Standard-Waagenprofils in Aeins angesprochen wird. 

In der jetzigen Ausbaustufe wird AeinsWiege durch eine XML gesteuert. 

Da hier mit XML hantiert wird werden statt der sonst aus dem WAM gewohnten &lt;STX>,&lt;ETX>,usw. eben [STX],[ETX],usw. verwendet.

Interessant sind u.a. 3 neue "Direktiven":

1. BCC

Vom WAM her dürfte schon die Möglichkeit bekannt sein die Prüfsumme für ausgehende Kommandos zu berechnen. AeinsWiege kann das auch für eingehende Waagendaten. Das ist wichtig, da manche Protokolle nach Sendung der Wiegedaten ( die mit BCC von der Waage aus gesendet werden ) noch eine positive Quittierung brauchen. Es ist einfach viel geschickter, wenn AeinsWiege diese BCC auch geprüft hat und dann entsprechend verfährt!

Beispiel:

```xml
<Sequence Send="[SOH]Ap[ENQ]"
Expect="[SOH]A[STX]([DATA][ETX])[BCC]"
Result="1"/>
```

Hinweis: Ausgehend - also im Send - ist die %BCC%-Syntax, eingehend - also im Expect - ist die [BCC]-Syntax.

2. DATA

Steht für eine beliebige Anzahl von unbekannten Zeichen.

Beispiel hierfür ist die obige Sequenz. AeinsWiege schickt an die Waage "[SOH]Ap[ENQ]", die Waage sendet daraufhin "[SOH]A[STX]QA[SP]0016060000000025140206144407[ETX]6" zurück. Der gesamte Anteil zwischen [STX]…[ETX] entspricht dabei ohne dass STX und ETX eben DATA, wobei die Prüfsumme von der Waage über [DATA] incl. [ETX] gebildet wurde und in diesem Falle 6 ist.

3. DATE

Mit Hilfe dieser Konstruktion lassen sich formatierte Zeitwerte innerhalb eines Send-Strings definieren.

Beispiel:

```xml
<Sequence
Send="[SOH]A[STX](ZSD{ddMMyyyyHHmm}%DATE%[ETX])%BCC%[ENQ]"
Expect="[ACK]" Wait="1000" />
```

Innerhalb der geschweiften Klammern steht der Format-String für die C#-Funktion ToString von DateTime-Objekt.

Alle Möglichkeiten hierzu lassen sich am besten übers Internet recherchieren.

In diesem Beispiel bedeutet:

| Format | Bedeutung |
| --- | --- |
| dd | Tagesangabe 2-stellig |
| MM | Monatsangabe 2-stellig |
| yyyy | Jahresangabe 4-stellig |
| HH | Stundenangabe 2-stellig |
| mm | Minutenangabe 2-stellig |

XML-Session-Beschreibung:

| Session-Attribut | Beispiel | Anmerkung |
| --- | --- | --- |
| PortName | COM1 | Legt den Anschluss für die Kommunikation fest, einschließlich<br>aller verfügbaren COM-Anschlüsse, aber nicht beschränkt auf diese.<br> <br>Ausnahmen:<br> System.ArgumentException:<br>Die System.IO.Ports.SerialPort.PortName-Eigenschaft wurde auf einen Wert mit einer Länge von 0 festgelegt.<br><ul><li>-&nbsp;oder&nbsp;-<br>Die System.IO.Ports.SerialPort.PortName-Eigenschaft wurde auf einen Wert festgelegt, der mit "\\" beginnt.</li><li>-&nbsp;oder&nbsp;-<br>Der Anschlussname war nicht gültig.<br>&nbsp;&nbsp; <br>System.ArgumentNullException:<br>&nbsp; Die System.IO.Ports.SerialPort.PortName-Eigenschaft wurde auf null festgelegt.<br>&nbsp;<br>System.InvalidOperationException:<br>&nbsp;&nbsp; Der angegebene Anschluss ist offen.</li></ul> |
| BaudRate | 9600 | Legt die serielle Baudrate fest.<br> <br>Ausnahmen:<br> System.ArgumentOutOfRangeException:<br>Die angegebene Baudrate ist kleiner oder gleich 0 oder größer als die maximal für das Gerät zulässige Baudrate.<br> System.IO.IOException:<br>Der Anschluss befindet sich in einem ungültigen Zustand.<br><ul><li>-&nbsp;oder&nbsp;- <br>Fehler&nbsp; beim Versuch, den Zustand des zugrunde liegenden Anschlusses festzulegen.<br>Beispielsweise waren die von diesem System.IO.Ports.SerialPort-Objekt übergebenen Parameter ungültig.</li></ul> |
| DataBits | 8 | Legt die Standardlänge der Datenbits pro Byte fest.<br> <br>Ausnahmen:<br> System.IO.IOException:<br>Der Anschluss befindet sich in einem ungültigen Zustand.<br><ul><li>-&nbsp;oder&nbsp;-<br>Fehler beim Versuch, den Zustand des zugrunde liegenden Anschlusses festzulegen. Beispielsweise&nbsp; waren die von diesem System.IO.Ports.SerialPort-Objekt übergebenen Parameter&nbsp; ungültig.<br>&nbsp;&nbsp;&nbsp; <br>&nbsp; System.ArgumentOutOfRangeException:<br>Der Datenbitwert ist kleiner als&nbsp;5 oder höher als&nbsp;8.</li></ul> |
| StopBits | 1 | Legt die Standardanzahl von Stoppbits pro Byte fest.<br> <br>Ausnahmen:<br> System.ArgumentOutOfRangeException:<br>Der System.IO.Ports.SerialPort.StopBits-Wert ist keiner der Werte aus der System.IO.Ports.StopBits-Enumeration.<br> <br> System.IO.IOException:<br>Der Anschluss befindet sich in einem ungültigen Zustand.<br><ul><li>-&nbsp;oder&nbsp;- <br>Fehler beim Versuch, den Zustand des zugrunde liegenden Anschlusses festzulegen.<br>Beispielsweise waren die von diesem System.IO.Ports.SerialPort-Objekt übergebenen Parameter ungültig.</li></ul> |
| Parity<br> <br>(Parität) | 2<br> <br> <br>0-None<br>1-Odd<br>2-Even<br>3-Mark<br>4-Space | Legt das Paritätsprüfungsprotokoll fest.<br> <br>Ausnahmen:<br> System.IO.IOException:<br>Der Anschluss befindet sich in einem ungültigen Zustand.<br><ul><li>-&nbsp;oder&nbsp;- <br>Fehler&nbsp; beim Versuch, den Zustand des zugrunde liegenden Anschlusses festzulegen. Beispielsweise waren die von diesem System.IO.Ports.SerialPort-Objekt übergebenen Parameter ungültig.<br>&nbsp; System.ArgumentOutOfRangeException:<br>Der übergebene System.IO.Ports.SerialPort.Parity-Wert ist kein gültiger Wert in der System.IO.Ports.Parity-Enumeration.</li></ul> |
| Technik | Seriell | Standard! Schaltet in den seriellen Modus, d.h. die Kommunikation geht über serielle Ports |
| Technik | Tcpip | Verwendet stattdessen TCPIP-Methoden |
| ServerIP | 192.168.241.19 | Ip des Hostes |
| ServerPort | 4711 | Port des Hostes |
| ReceiveTimeout | 0 | Gleichnamiges Socket-Property.<br>Timeout-Wert in Millisekunden (Standard ist 0, das ist gleichbedeutend mit unendlicher Wartezeit – ebenso wie -1)<br>Siehe auch [Socket.ReceiveTimeout Property](https://msdn.microsoft.com/en-us/library/system.net.sockets.socket.receivetimeout%28v=vs.110%29.aspx). |
| Continue | | Standard ist false<br>Globale Setzung hier überschreibt gleichnamiges Sequence-Feature [Continue](./standardwaagenprofil_unterstuetzung_aeinswiege.md#xmlcontinue) |
| Result | | Standard ist false<br>Globale Setzung hier überschreibt gleichnamiges Sequence-Feature [Result](./standardwaagenprofil_unterstuetzung_aeinswiege.md#xmlresult) |

XML-Sequence-Beschreibung

| Sequence-Attribut | Beispiel | Anmerkung |
| --- | --- | --- |
| Send | [SOH]A[STX](WA[ETX])%BCC%[ENQ] | Hier Beispiel für abgehende BCC-Berechnung |
| Expect | [EOT] | Antwort der Waage |
| Retry | [ACK] | Wenn der Wert für **Expect** nicht empfangen wurde, dann besteht die Möglichkeit einer Wiederholung der Sequenz bei der vorher nochmal der Wert von **Retry** gesendet wird.<br> <br>Standard ist leer.<br> <br>In diesem Beispiel bedeutet das, wenn das erwartete [**EOT**] nicht kam, dann wird [**ACK**] gesendet und danach nochmal der Inhalt von Send.<br> <br>Dieses wird **Tries**\-Mal wiederholt bevor abgebrochen wird. |
| Tries | 10 | Siehe Erläuterungen bei **Retry**.<br> <br>Standard ist 10. |
| Wait | 10000 | Anzahl der Millisekunden, die gewartet werden, soll bis ein Ergebnis aufgelaufen ist.<br>Es kann sein das eine Waage nur einen Teil der "Expected" schickt bis zum Timeout, oder gar nichts, was fast dasselbe ist. Dann ist die Wiegung fehlgeschlagen.<br>Es kann sein, das weit vor dem Timeout die Waagenrückgabe schon anders als erwartet ist, dann ist auch fehlgeschlagen …<br>Genauso gut kann es aber sein das die Waage schon erheblich früher die erwartete Antwort gesendet hat, dann ist ja alles gut und es geht sofort weiter.<br>(Standard ist 100 Millisekunden) |
| Pause | 10000 | So viele Millisekunden werden **nach Ausführung einer Sendung und eines Empfangs** pausiert. (Standard ist 0 Millisekunden) |
| Delay | 500 | So viele Millisekunden werden nach Senden gewartet, bevor Empfang abgefragt wird. (Standard: 500 Millisekunden) |
| Result | 1 | Eine so ausgewiesene Sequence liefert Daten der Wiegung.<br> <br>Dabei können mehrere Sequencen mit Result=1 gekennzeichnet werden. Die Daten werden dann aneinandergereiht (konkateniert).<br> <br>(Standard: 0 )<br> |
| Continue | 1 | Es gibt Fälle in denen Waagen/Wiegesysteme kein Ende-Kennzeichen bzw. Begrenzer senden, aber trotzdem noch eine abschließende Quittung benötigen. Mit Hilfe der Continue-Anweisung und dem DATA-Konstrukt lässt sich so ein Workaround schaffen, indem man die DATA-Anweisung so formuliert das sie die zu erwartende Rückgabe aufnimmt und Continue=1 AeinsWiege anweist weiter zu machen.<br> <br>Beispiel:<br>([DATA]X)<br> <br>Wobei man sicherstellen muss das das X nicht vorkommt. AeinsWiege wird bis zum Timeout versuchen das X zubekommen – was aber nicht kommen kann und normalerweise mit einem nicht erfolgreichen Abbruch quittiert wird.<br>Mit Continue=1 wird dann aber weiter gemacht! |

| Kommandozeilen-parameter | |
| --- | --- |
| 0 | 0: Normalmodus<br>1: Gui-Modus (dieser Modus ist noch Entwicklung und wird bei Anschluss neuer<br>Waagensysteme über dieses Programm entsprechend weiterentwicklet)<br>Beispiel für Start im Analyse-Modus: Aeinswiege 1<br>Beispiel für Start im Aeins-Modus:<br><pre><code>' funktionsfähiges Standard-Script für das&#10; Standardwaagenprofil AMIC STANDARDWAAGE.&#10;Der Zubringer erzeugt&#10; zufällige Wiegeergebnisse&#10; &#10;' das Kundenergebnis&#10; wird standardmäßig im A.eins-Import-Verzeichnis erwartet und heisst&#10;&#10;standardmäßig&#10; AMIC_STANDARDWAAGE.TXT&#10;' Erzeugt wird das&#10; Kundenergebnis durch ein Script/ einen Programmaufruf der die Wiegung&#10; initiert und standardmäßig das Ergebnis in der Form&#10;' Gewicht;Wiegnummer&#10; zurückliefert.&#10;' Für das&#10; AMIC_STANDARDWAAGE - Profil wird eine kleine Demo-Applikation (&#10; amic_standardwaage.exe ) im Bin-Verzeichnis mit ausgeliefert&#10; &#10;' Schritt1 :&#10; Waageinitiierung&#10;'&#10; --------------------------------------------------------------------&#10;' Bestimmung der Lage&#10; des Kundenergebnisses&#10;'&#10; --------------------------------------------------------------------&#10;dim&#10; kundenergebnis&#10; &#10;dim&#10; aeinsrootverz&#10;aeinsrootverz =&#10; Aeins.param( "ahoiroot" )&#10;kundenergebnis =&#10; aeinsrootverz &amp; "\import" &amp; "\" &amp;&#10; "AMIC_STANDARDWAAGE.TXT"&#10;kundenergebnis =&#10; "C:\Users\ah\Documents\Entwicklung\Kairo\Aeins\A.eins.Net&#10;\Support\ComportTester\ComportTester\bin\Debug\aeinswiege.txt"&#10; &#10;dim&#10; xmlsteuerdatei&#10;xmlsteuerdatei =&#10; "C:\Users\ah\Documents\Entwicklung\Kairo\Aeins\A.eins.Net&#10;\Support\ComportTester\ComportTester\bin\Debug\aeinswiege.xml"&#10; &#10;'msgbox&#10; kundenergebnis&#10; &#10;'&#10; --------------------------------------------------------------------&#10;' Löschen eines evtl.&#10; vorherigen Kundenergebnisses&#10;'&#10; --------------------------------------------------------------------&#10;dim hdl&#10;hdl = "fso"&#10;Aeins.jpp_new hdl ,&#10; "JFileSystem"&#10;Aeins.jpp_in hdl&#10; , "file" , kundenergebnis&#10;Aeins.jpp_ex hdl&#10; , "DeleteFile"&#10;Aeins.jpp_delete&#10; hdl&#10; &#10;'&#10; --------------------------------------------------------------------&#10;' Veranlassen des&#10; neuen Kundenergebnisses&#10;'&#10; --------------------------------------------------------------------&#10;dim fso:set fso =&#10; CreateObject("Scripting.FileSystemObject")&#10;dim&#10; wiege_client&#10;dim&#10; aeinsverz&#10;aeinsverz =&#10; Aeins.param( "exepath" )&#10;wiege_client =&#10; "C:\Users\ah\Documents\Entwicklung\Kairo\Aeins\A.eins.Net&#10;\Support\ComportTester\ComportTester\bin\Debug" &amp;&#10; "\aeinswiege.exe 0 " &amp; xmlsteuerdatei &amp; " " &amp;&#10; kundenergebnis&#10; &#10;ExecuteDOS wiege_client , 0 ,&#10;1</code></pre> |
| 1 | Location der Aeinswiege.xml. Ist keine angegeben, dann ist es standardmäßig Aeinswiege.xml |
| 2 | Location der Aeinswiege.txt. Ist keine angegeben, dann ist es standardmäßig<br>Aeinswiege.txt |

Beispiel-Sitzung:

| | |
| --- | --- |
| XML-Steuerdatei | <pre><code>&lt;?xml version="1.0" encoding="utf-8"&#10; ?&gt;&#10;&lt;!--&#10;&#10; PR&#10; 1613&#10;--&gt;&#10;&lt;Session&#10; ServerIP="192.168.241.93"&#10; &#10; ServerPort="950"&#10; &#10;&#10; &#10; Technik="tcpip"&#10; &#10;&#10; &#10; PortName="COM8"&#10; &#10; BaudRate="4800"&#10; &#10; DataBits="7"&#10; &#10; StopBits="1"&#10; &#10; Parity="2"&#10; &#10; &#10; Logging="log.txt"&#10;&gt; &#10; &#10; &lt;Sequences&gt;&#10; &lt;!-- Pollen bis EOT kommt&#10; --&gt;&#10; &lt;Sequence Send="[SOH]Ap[ENQ]"&#10; Expect="[EOT]" Retry="[ACK]" /&gt;&#10; &lt;Sequence Send="[EOT]"&#10; /&gt;&#10; &#10; &lt;!-- Gewichtsanforderung&#10; --&gt;&#10; &lt;Sequence&#10; Send="[SOH]A[STX]WGA[ETX]R[ENQ]" Expect="[ACK]" /&gt;&#10; &lt;Sequence Send="[EOT]"&#10; /&gt;&#10; &lt;Sequence Send="[SOH]Ap[ENQ]"&#10; Expect="[SOH]A[STX]([DATA][ETX])[BCC]" Wait="10000"&#10; Result="1"/&gt;&#10; &lt;Sequence Send="[ACK]"&#10; /&gt;&#10; &lt;Sequence Send="[SOH]Ap[ENQ]"&#10; Expect="[EOT]" Retry="[ACK]" /&gt;&#10; &#10; &lt;!-- ZSD&#10;--&gt;&#10; &lt;Sequence&#10; Send="[SOH]A[STX](ZSD{ddMMyyyyHHmm}%DATE%[ETX])%BCC%[ENQ]" Expect="[ACK]"&#10; /&gt;&#10; &lt;Sequence Send="[SOH]Ap[ENQ]"&#10; Expect="[SOH]A[STX](QZSD[ETX])[BCC]" /&gt;&#10; &lt;Sequence Send="[ACK]"&#10; /&gt;&#10; &lt;Sequence Send="[SOH]Ap[ENQ]"&#10; Expect="[EOT]" Retry="[ACK]" /&gt;&#10; &#10; &lt;!-- ZVWA&#10; --&gt;&#10; &lt;Sequence&#10; Send="[SOH]A[STX](ZVWA 0000011[ETX])%BCC%[ENQ]"&#10; Expect="[ACK]" /&gt;&#10; &lt;Sequence Send="[SOH]Ap[ENQ]"&#10; Expect="[SOH]A[STX](Q[ETX])[BCC]" /&gt;&#10; &lt;Sequence Send="[ACK]"&#10; /&gt;&#10; &lt;Sequence Send="[SOH]Ap[ENQ]"&#10; Expect="[EOT]" Retry="[ACK]" /&gt;&#10; &#10; &lt;!-- ZVSEQ&#10; --&gt;&#10; &lt;Sequence&#10; Send="[SOH]A[STX](ZVSEQ [ETX])%BCC%[ENQ]" Expect="[ACK]"&#10; /&gt;&#10; &lt;Sequence Send="[SOH]Ap[ENQ]"&#10; Expect="[SOH]A[STX](QVSEQ[DATA][ETX])[BCC]" Result="1"/&gt;&#10; &lt;Sequence Send="[ACK]"&#10; /&gt;&#10; &lt;Sequence Send="[SOH]Ap[ENQ]"&#10; Expect="[EOT]" Retry="[ACK]" /&gt;&#10; &#10; &lt;!-- BSPRINT&#10; --&gt;&#10; &lt;Sequence&#10; Send="[SOH]A[STX](BSPRINT [ETX])%BCC%[ENQ]" Expect="[ACK]"&#10; /&gt;&#10; &lt;Sequence Send="[SOH]Ap[ENQ]"&#10; Expect="[SOH]A[STX](Q[ETX])[BCC]" /&gt;&#10; &lt;Sequence Send="[ACK]"&#10; /&gt;&#10; &lt;Sequence Send="[SOH]Ap[ENQ]"&#10; Expect="[EOT]" Retry="[ACK]" /&gt;&#10; &#10; &lt;!-- ZVFA&#10; --&gt;&#10; &lt;Sequence&#10; Send="[SOH]A[STX](ZVFA [ETX])%BCC%[ENQ]" Expect="[ACK]"&#10; /&gt;&#10; &lt;Sequence Send="[SOH]Ap[ENQ]"&#10; Expect="[SOH]A[STX](QVFA[DATA][ETX])[BCC]" /&gt;&#10; &lt;Sequence Send="[ACK]"&#10; /&gt;&#10; &lt;Sequence Send="[SOH]Ap[ENQ]"&#10; Expect="[EOT]" Retry="[ACK]" /&gt;&#10; &#10; &lt;!-- AZM&#10;--&gt;&#10; &lt;Sequence Send="[SOH]AZM"&#10; Expect="[EOT]" /&gt;&#10; &lt;Sequence Send="[SOH]Ap[ENQ]"&#10; Expect="[EOT]" Retry="[ACK]" /&gt;&#10; &#10; &#10; &lt;/Sequences&gt;&#10;&lt;/Session&gt;</code></pre> |
| Beispiel-LOG | <pre><code>Timestamp &#10; &#124;Sended &#10; &#124;Readed &#10; &#124;Status &#124;Expected&#10;27.03.14&#10; 09:21:15.859&#124;[SOH]Ap[ENQ] &#10; &#124;[EOT] &#10; &#124;Exakt &#124;[EOT]&#10;27.03.14&#10; 09:21:17.073&#124;[EOT] &#10; &#124; &#10; &#124;Exakt &#124;&#10;27.03.14&#10; 09:21:17.576&#124;[SOH]A[STX]WGA[ETX]R[ENQ] &#10; &#124;[ACK] &#10; &#124;Exakt &#124;[ACK]&#10;27.03.14&#10; 09:21:18.078&#124;[EOT] &#10; &#124; &#10; &#124;Exakt &#124;&#10;27.03.14&#10; 09:21:19.348&#124;[SOH]Ap[ENQ] &#10; &#124;[SOH]A[STX]QGA[SP]00772610[ETX]q &#10; &#124;Exakt &#124;[SOH]A[STX]([DATA][ETX])[BCC]&#10;27.03.14&#10; 09:21:19.852&#124;[ACK] &#10; &#124; &#10; &#124;Exakt &#124;&#10;27.03.14&#10; 09:21:20.354&#124;[SOH]Ap[ENQ] &#10; &#124;[EOT] &#10; &#124;Exakt &#124;[EOT]&#10;27.03.14&#10; 09:21:20.857&#124;[SOH]A[STX]ZSD270320140921[ETX]E[ENQ] &#10; &#124;[ACK] &#10; &#124;Exakt &#124;[ACK]&#10;27.03.14&#10; 09:21:21.364&#124;[SOH]Ap[ENQ] &#10; &#124;[SOH]A[STX]QZSD[ETX][US] &#10; &#124;Exakt &#124;[SOH]A[STX](QZSD[ETX])[BCC]&#10;27.03.14&#10; 09:21:21.866&#124;[ACK] &#10; &#124; &#10; &#124;Exakt &#124;&#10;27.03.14&#10; 09:21:22.369&#124;[SOH]Ap[ENQ] &#10; &#124;[EOT] &#10; &#124;Exakt &#124;[EOT]&#10;27.03.14&#10; 09:21:22.870&#124;[SOH]A[STX]ZVWA[SP][SP][SP][SP]0000011[ETX])[ENQ]&#10; &#124;[ACK] &#10; &#124;Exakt &#124;[ACK]&#10;27.03.14&#10; 09:21:23.377&#124;[SOH]Ap[ENQ] &#10; &#124;[SOH]A[STX]Q[ETX]R &#10; &#124;Exakt &#124;[SOH]A[STX](Q[ETX])[BCC]&#10;27.03.14&#10; 09:21:23.879&#124;[ACK] &#10; &#124; &#10; &#124;Exakt &#124;&#10;27.03.14&#10; 09:21:24.381&#124;[SOH]Ap[ENQ] &#10; &#124;[EOT] &#10; &#124;Exakt &#124;[EOT]&#10;27.03.14&#10; 09:21:24.884&#124;[SOH]A[STX]ZVSEQ[SP][SP][SP][ETX]h[ENQ] &#10; &#124;[ACK] &#10; &#124;Exakt &#124;[ACK]&#10;27.03.14&#10; 09:21:25.394&#124;[SOH]Ap[ENQ] &#10; &#124;[SOH]A[STX]QVSEQ[SP][SP][SP][SP]0000000057[ETX]A &#10; &#124;Exakt &#124;[SOH]A[STX](QVSEQ[DATA][ETX])[BCC]&#10;27.03.14&#10; 09:21:25.897&#124;[ACK] &#10; &#124; &#10; &#124;Exakt &#124;&#10;27.03.14&#10; 09:21:26.399&#124;[SOH]Ap[ENQ] &#10; &#124;[EOT] &#10; &#124;Exakt &#124;[EOT]&#10;27.03.14&#10; 09:21:26.901&#124;[SOH]A[STX]BSPRINT[SP][ETX]c[ENQ] &#10; &#124;[ACK] &#10; &#124;Exakt &#124;[ACK]&#10;27.03.14&#10; 09:21:27.407&#124;[SOH]Ap[ENQ] &#10; &#124;[SOH]A[STX]Q[ETX]R &#10; &#124;Exakt &#124;[SOH]A[STX](Q[ETX])[BCC]&#10;27.03.14&#10; 09:21:27.909&#124;[ACK] &#10; &#124; &#10; &#124;Exakt &#124;&#10;27.03.14&#10; 09:21:28.412&#124;[SOH]Ap[ENQ] &#10; &#124;[EOT] &#10; &#124;Exakt &#124;[EOT]&#10;27.03.14&#10; 09:21:28.914&#124;[SOH]A[STX]ZVFA[SP][SP][SP][SP][ETX][BS][ENQ] &#10; &#124;[ACK] &#10; &#124;Exakt &#124;[ACK]&#10;27.03.14&#10; 09:21:29.423&#124;[SOH]Ap[ENQ] &#10; &#124;[SOH]A[STX]QVFA[SP][SP][SP][SP]0[ETX]3 &#10; &#124;Exakt &#124;[SOH]A[STX](QVFA[DATA][ETX])[BCC]&#10;27.03.14&#10; 09:21:29.925&#124;[ACK] &#10; &#124; &#10; &#124;Exakt &#124;&#10;27.03.14&#10; 09:21:30.427&#124;[SOH]Ap[ENQ] &#10; &#124;[EOT] &#10; &#124;Exakt &#124;[EOT]&#10;27.03.14&#10; 09:21:30.929&#124;[SOH]AZM &#10; &#124;[EOT] &#10; &#124;Exakt &#124;[EOT]&#10;27.03.14&#10; 09:21:31.431&#124;[SOH]Ap[ENQ] &#10; &#124;[EOT] &#10; &#124;Exakt &#124;[EOT]</code></pre> |
| Beispiel-Ergebnis | <code>&lt;SOH&gt;A&lt;STX&gt;QGA&lt;SP&gt;00772610&lt;ETX&gt;q&lt;SOH&gt;A&lt;STX&gt;QVSEQ&lt;SP&gt;&lt;SP&gt;&lt;SP&gt;&lt;SP&gt;0000000057&lt;ETX&gt;A</code> |
