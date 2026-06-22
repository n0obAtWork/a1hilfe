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
| PortName | COM1 | Legt den Anschluss für die Kommunikation fest, einschließlich<br>aller verfügbaren COM-Anschlüsse, aber nicht beschränkt auf diese.<br> <br>Ausnahmen:<br> System.ArgumentException:<br>Die System.IO.Ports.SerialPort.PortName-Eigenschaft wurde auf einen Wert mit einer Länge von 0 festgelegt.<br>\- oder -<br>Die System.IO.Ports.SerialPort.PortName-Eigenschaft wurde auf einen Wert festgelegt, der mit "\\\\" beginnt.<br>\- oder -<br>Der Anschlussname war nicht gültig.<br> <br>System.ArgumentNullException:<br> Die System.IO.Ports.SerialPort.PortName-Eigenschaft wurde auf null festgelegt.<br> <br>System.InvalidOperationException:<br> Der angegebene Anschluss ist offen. |
| BaudRate | 9600 | Legt die serielle Baudrate fest.<br> <br>Ausnahmen:<br> System.ArgumentOutOfRangeException:<br>Die angegebene Baudrate ist kleiner oder gleich 0 oder größer als die maximal für das Gerät zulässige Baudrate.<br> System.IO.IOException:<br>Der Anschluss befindet sich in einem ungültigen Zustand.<br>\- oder -<br>Fehler beim Versuch, den Zustand des zugrunde liegenden Anschlusses festzulegen.<br>Beispielsweise waren die von diesem System.IO.Ports.SerialPort-Objekt übergebenen Parameter ungültig. |
| DataBits | 8 | Legt die Standardlänge der Datenbits pro Byte fest.<br> <br>Ausnahmen:<br> System.IO.IOException:<br>Der Anschluss befindet sich in einem ungültigen Zustand.<br>\- oder -<br>Fehler beim Versuch, den Zustand des zugrunde liegenden Anschlusses festzulegen. Beispielsweise waren die von diesem System.IO.Ports.SerialPort-Objekt übergebenen Parameter ungültig.<br> <br> System.ArgumentOutOfRangeException:<br>Der Datenbitwert ist kleiner als 5 oder höher als 8. |
| StopBits | 1 | Legt die Standardanzahl von Stoppbits pro Byte fest.<br> <br>Ausnahmen:<br> System.ArgumentOutOfRangeException:<br>Der System.IO.Ports.SerialPort.StopBits-Wert ist keiner der Werte aus der System.IO.Ports.StopBits-Enumeration.<br> <br> System.IO.IOException:<br>Der Anschluss befindet sich in einem ungültigen Zustand.<br>\- oder -<br>Fehler beim Versuch, den Zustand des zugrunde liegenden Anschlusses festzulegen.<br>Beispielsweise waren die von diesem System.IO.Ports.SerialPort-Objekt übergebenen Parameter ungültig. |
| Parity<br> <br>(Parität) | 2<br> <br> <br>0-None<br>1-Odd<br>2-Even<br>3-Mark<br>4-Space | Legt das Paritätsprüfungsprotokoll fest.<br> <br>Ausnahmen:<br> System.IO.IOException:<br>Der Anschluss befindet sich in einem ungültigen Zustand.<br>\- oder -<br>Fehler beim Versuch, den Zustand des zugrunde liegenden Anschlusses festzulegen. Beispielsweise waren die von diesem System.IO.Ports.SerialPort-Objekt übergebenen Parameter ungültig.<br> System.ArgumentOutOfRangeException:<br>Der übergebene System.IO.Ports.SerialPort.Parity-Wert ist kein gültiger Wert in der System.IO.Ports.Parity-Enumeration. |
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
| 0 | 0: Normalmodus<br>1: Gui-Modus (dieser Modus ist noch Entwicklung und wird bei Anschluss neuer<br>Waagensysteme über dieses Programm entsprechend weiterentwicklet)<br>Beispiel für Start im Analyse-Modus: Aeinswiege 1<br>Beispiel für Start im Aeins-Modus: |
| 1 | Location der Aeinswiege.xml. Ist keine angegeben, dann ist es standardmäßig Aeinswiege.xml |
| 2 | Location der Aeinswiege.txt. Ist keine angegeben, dann ist es standardmäßig<br>Aeinswiege.txt |

Beispiel-Sitzung:

| | 
 |
| --- | --- |
| XML-Steuerdatei | 

```xml
<?xml version="1.0" encoding="utf-8"
      ?>
<!--

  PR
      1613
-->
<Session
      ServerIP="192.168.241.93"

      ServerPort="950"

      Technik="tcpip"

      PortName="COM8"

      BaudRate="4800"

      DataBits="7"

      StopBits="1"

      Parity="2"

      Logging="log.txt"
>

      <Sequences>
    <!-- Pollen bis EOT kommt
      -->
    <Sequence Send="[SOH]Ap[ENQ]"
      Expect="[EOT]" Retry="[ACK]" />
    <Sequence Send="[EOT]"
      />

    <!-- Gewichtsanforderung
      -->
    <Sequence
      Send="[SOH]A[STX]WGA[ETX]R[ENQ]" Expect="[ACK]" />
    <Sequence Send="[EOT]"
      />
    <Sequence Send="[SOH]Ap[ENQ]"
      Expect="[SOH]A[STX]([DATA][ETX])[BCC]" Wait="10000"
      Result="1"/>
    <Sequence Send="[ACK]"
      />
    <Sequence Send="[SOH]Ap[ENQ]"
      Expect="[EOT]" Retry="[ACK]" />

    <!-- ZSD
-->
    <Sequence
      Send="[SOH]A[STX](ZSD{ddMMyyyyHHmm}%DATE%[ETX])%BCC%[ENQ]" Expect="[ACK]"
      />
    <Sequence Send="[SOH]Ap[ENQ]"
      Expect="[SOH]A[STX](QZSD[ETX])[BCC]" />
    <Sequence Send="[ACK]"
      />
    <Sequence Send="[SOH]Ap[ENQ]"
      Expect="[EOT]" Retry="[ACK]" />

    <!-- ZVWA
      -->
    <Sequence
      Send="[SOH]A[STX](ZVWA    0000011[ETX])%BCC%[ENQ]"
      Expect="[ACK]" />
    <Sequence Send="[SOH]Ap[ENQ]"
      Expect="[SOH]A[STX](Q[ETX])[BCC]" />
    <Sequence Send="[ACK]"
      />
    <Sequence Send="[SOH]Ap[ENQ]"
      Expect="[EOT]" Retry="[ACK]" />

    <!-- ZVSEQ
      -->
    <Sequence
      Send="[SOH]A[STX](ZVSEQ   [ETX])%BCC%[ENQ]" Expect="[ACK]"
      />
    <Sequence Send="[SOH]Ap[ENQ]"
      Expect="[SOH]A[STX](QVSEQ[DATA][ETX])[BCC]" Result="1"/>
    <Sequence Send="[ACK]"
      />
    <Sequence Send="[SOH]Ap[ENQ]"
      Expect="[EOT]" Retry="[ACK]" />

    <!-- BSPRINT
      -->
    <Sequence
      Send="[SOH]A[STX](BSPRINT [ETX])%BCC%[ENQ]" Expect="[ACK]"
      />
    <Sequence Send="[SOH]Ap[ENQ]"
      Expect="[SOH]A[STX](Q[ETX])[BCC]" />
    <Sequence Send="[ACK]"
      />
    <Sequence Send="[SOH]Ap[ENQ]"
      Expect="[EOT]" Retry="[ACK]" />

    <!-- ZVFA
      -->
    <Sequence
      Send="[SOH]A[STX](ZVFA    [ETX])%BCC%[ENQ]" Expect="[ACK]"
      />
    <Sequence Send="[SOH]Ap[ENQ]"
      Expect="[SOH]A[STX](QVFA[DATA][ETX])[BCC]" />
    <Sequence Send="[ACK]"
      />
    <Sequence Send="[SOH]Ap[ENQ]"
      Expect="[EOT]" Retry="[ACK]" />

    <!-- AZM
-->
    <Sequence Send="[SOH]AZM"
      Expect="[EOT]" />
    <Sequence Send="[SOH]Ap[ENQ]"
      Expect="[EOT]" Retry="[ACK]" />

      </Sequences>
</Session>
```

 |
| Beispiel-LOG | 

```text
Timestamp
      |Sended
      |Readed
      |Status   |Expected
27.03.14
      09:21:15.859|[SOH]Ap[ENQ]
      |[EOT]
      |Exakt    |[EOT]
27.03.14
      09:21:17.073|[EOT]
      |
      |Exakt    |
27.03.14
      09:21:17.576|[SOH]A[STX]WGA[ETX]R[ENQ]
      |[ACK]
      |Exakt    |[ACK]
27.03.14
      09:21:18.078|[EOT]
      |
      |Exakt    |
27.03.14
      09:21:19.348|[SOH]Ap[ENQ]
      |[SOH]A[STX]QGA[SP]00772610[ETX]q
      |Exakt    |[SOH]A[STX]([DATA][ETX])[BCC]
27.03.14
      09:21:19.852|[ACK]
      |
      |Exakt    |
27.03.14
      09:21:20.354|[SOH]Ap[ENQ]
      |[EOT]
      |Exakt    |[EOT]
27.03.14
      09:21:20.857|[SOH]A[STX]ZSD270320140921[ETX]E[ENQ]
      |[ACK]
      |Exakt    |[ACK]
27.03.14
      09:21:21.364|[SOH]Ap[ENQ]
      |[SOH]A[STX]QZSD[ETX][US]
      |Exakt    |[SOH]A[STX](QZSD[ETX])[BCC]
27.03.14
      09:21:21.866|[ACK]
      |
      |Exakt    |
27.03.14
      09:21:22.369|[SOH]Ap[ENQ]
      |[EOT]
      |Exakt    |[EOT]
27.03.14
      09:21:22.870|[SOH]A[STX]ZVWA[SP][SP][SP][SP]0000011[ETX])[ENQ]
      |[ACK]
      |Exakt    |[ACK]
27.03.14
      09:21:23.377|[SOH]Ap[ENQ]
      |[SOH]A[STX]Q[ETX]R
      |Exakt    |[SOH]A[STX](Q[ETX])[BCC]
27.03.14
      09:21:23.879|[ACK]
      |
      |Exakt    |
27.03.14
      09:21:24.381|[SOH]Ap[ENQ]
      |[EOT]
      |Exakt    |[EOT]
27.03.14
      09:21:24.884|[SOH]A[STX]ZVSEQ[SP][SP][SP][ETX]h[ENQ]
      |[ACK]
      |Exakt    |[ACK]
27.03.14
      09:21:25.394|[SOH]Ap[ENQ]
      |[SOH]A[STX]QVSEQ[SP][SP][SP][SP]0000000057[ETX]A
      |Exakt    |[SOH]A[STX](QVSEQ[DATA][ETX])[BCC]
27.03.14
      09:21:25.897|[ACK]
      |
      |Exakt    |
27.03.14
      09:21:26.399|[SOH]Ap[ENQ]
      |[EOT]
      |Exakt    |[EOT]
27.03.14
      09:21:26.901|[SOH]A[STX]BSPRINT[SP][ETX]c[ENQ]
      |[ACK]
      |Exakt    |[ACK]
27.03.14
      09:21:27.407|[SOH]Ap[ENQ]
      |[SOH]A[STX]Q[ETX]R
      |Exakt    |[SOH]A[STX](Q[ETX])[BCC]
27.03.14
      09:21:27.909|[ACK]
      |
      |Exakt    |
27.03.14
      09:21:28.412|[SOH]Ap[ENQ]
      |[EOT]
      |Exakt    |[EOT]
27.03.14
      09:21:28.914|[SOH]A[STX]ZVFA[SP][SP][SP][SP][ETX][BS][ENQ]
      |[ACK]
      |Exakt    |[ACK]
27.03.14
      09:21:29.423|[SOH]Ap[ENQ]
      |[SOH]A[STX]QVFA[SP][SP][SP][SP]0[ETX]3
      |Exakt    |[SOH]A[STX](QVFA[DATA][ETX])[BCC]
27.03.14
      09:21:29.925|[ACK]
      |
      |Exakt    |
27.03.14
      09:21:30.427|[SOH]Ap[ENQ]
      |[EOT]
      |Exakt    |[EOT]
27.03.14
      09:21:30.929|[SOH]AZM
      |[EOT]
      |Exakt    |[EOT]
27.03.14
      09:21:31.431|[SOH]Ap[ENQ]
      |[EOT]
      |Exakt    |[EOT]
```

 |
| Beispiel-Ergebnis | 

```text
<SOH>A<STX>QGA<SP>00772610<ETX>q<SOH>A<STX>QVSEQ<SP><SP><SP><SP>0000000057<ETX>A
```

 |
