# Erlöskennziffer / Kontozuordnung

<!-- source: https://amic.de/hilfe/_erlskennzifferkontoz.htm -->

Hauptmenü > Administration > Erlöskennziffern > Erlöskennziffer/Kontozuordnung

oder Direktsprung **[EKZZ]**

Hier erfolgt die Verknüpfung der Elemente

- Erlöskennziffer
- Gültigkeit der Eintragungen
- Steuerschlüssel
- Erlösklasse
- Steuergruppe
- Buchungsklasse

mit den Konten der Finanzbuchhaltung. Hier kann man die Bearbeitung wie bei der normalen Stammdatenpflege Datensatz für Datensatz vornehmen oder aber ganze Gruppen von Datensätzen gleichzeitig ändern. Für die gleichzeitige Bearbeitung der Datensätze kann man unter „gültig ab“ in den Feldern Steuerschlüssel, Erlösklasse bzw. Steuergruppe einen Haken setzen.

![](../../../ImagesExt/image8_30.jpg)

Setzt man z.B. beim Steuerschlüssel den Haken, so werden in der Datentabelle alle möglichen Kombinationen für Erlösklasse und Steuergruppe angezeigt. In der so entsehenden Übersicht kann man schnell erkennen, wenn Konten falsch zugeordnet sind. Die Felder rechts von den Haken geben die Sortierungsreihenfolge an. Sie wird immer in der Reihenfolge gesetzt, in der man die Haken setzt.

Die Schlüsselfelder Steuerschlüssel, Erlösklasse und Steuergruppe links sind auch im Ändern-Fall aktiv, wobei man jedoch nicht die Werte ändert, sondern die anzuzeigenden Daten auswählen kann.

<details>
<summary>Felder der Erlöskennziffer / Kontozuordnung:</summary>

| Felder |
| --- |
| EKZ Nummer | Die Erlöskennziffer, die im Artikel hinterlegt ist. |
| Gültig ab | Mit Hilfe der Angabe eines Datums hat man die Möglichkeit zukünftige Änderungen der Konten für die Kombination aus EKZ Nummer, Erlösklasse, Steuerschlüssel und Buchklasse vorab in die Datenbank einzupflegen um dann zum entsprechenden Datum Buchungen auf den richtigen Konten zu erhalten. |
| Steuerschlüssel | Es ist möglich, Erlöse nach steuerlichen Gesichtspunkten zu differenzieren (Verprobung Umsatzsteuervoranmeldung). Die Definition der Steuerschlüssel erfolgt bekannt­lich im Rahmen der Firmenkonstanten unter dem Punkt Steuerschlüssel. Der Steuerschlüssel wird im Artikelstamm hinterlegt. Der Steuerschlüssel 0 (Null) hat für das Erlöskennzifferwesen DEFAULT-Funktion. Daher sollte er nicht als 0,00 % Steuer definiert werden. |
| Erlösklasse | Kundenspezifische Zuordnung. |
| Steuergruppe | |
| Buchklasse | Per Buchklasse werden unterschiedliche Buchungstypen auf verschiedene Erlös- und Aufwandskonten gelenkt. Buchklassen sind in A.eins festgelegt und können nicht geändert oder erweitert werden.<br><br><br>Bedeutung der Buchklasse:<br> 0: DEFAULT Fehlwert (alles andere)<br> 1: Normal-Buchungen (Waren-Ein- oder -Verkauf)<br> 2: Erlös-/Aufwandsschmälerungen<br> Hierunter laufen folgende Warenpositionsmechanismen:<br><ul><li>&nbsp;- sämtliche Zu- / Abschläge</li><li>&nbsp;- sämtliche Rabatte<br>&nbsp;3. Frachten<br>&nbsp;5. Gutschriften<br>&nbsp;Hierunter fallen die Vorgangsklassen 800 und 1800.<br>&nbsp;Gutschriften lassen sich somit buchungstechnisch anders <br>&nbsp;behandeln als Storno-Belege. <br>Die Buchungsmaschine arbeitet bzgl. der Buchungsklasse nach folgender Logik:<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; 5 -&gt; 1 -&gt; 0<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; 3 -&gt; 1 -&gt; 0<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; 2 -&gt; 1 -&gt; 0<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; 1 -&gt; 0<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; 0<br>Beispiel:<br>Ist für die Buchklasse 5 (Gutschrift) eine explizite Kontenzuordnung definiert, so wird diese verwendet. Falls nicht, wird eine solche in Buchklasse 1 gesucht. Endet auch dort die Suche ohne Erfolg, so findet die Fehlwerteinstellung 0 Anwendung.</li></ul> |
| Erlöskonto | Erlöskonto, auf dem die Verkäufe verbucht werden sollen. |
| Aufwandskonto | Aufwandskonto, auf dem die Einkäufe verbucht werden sollen |

</details>

<details>
<summary>Felder des Kontoregister</summary>

Diese Felder werden nur angezeigt, wenn der Steuerparameter ([Steuerparameter 720](../../steuerparameter/optionen_warenwirtschaft/mengenbuchung_bei_fibu_uebertrag_spa_720.md) „Mengenbuchung bei dem Übertrag in die Finanzbuchhaltung“) entsprechend gesetzt ist. Das Bestands-Erlöskonto und das Bestands-Aufwandskonto kann im Sachkontenstamm fest einem zugehörigen Erlös, oder Aufwandskonto zugewiesen werden. Es ist dann hier nicht mehr änderbar, sondern nur im [Sachkontenstamm](../../../finanzbuchhaltung/stammdaten_der_fibu/sachkonten.md#ZugehörigesStatistikkonto).

| Bestandskonten für Mengenbuchung beim Übertrag in die Finanzbuchhaltung |
| --- |
| Erlöskonto | Konto auf dem die Mengen für den Verkauf gebucht werden sollen |
| Aufwandskonto | Konto auf dem die Mengen für den Einkauf gebucht werden sollen |
| Erlössammelkonto | Gegenkonto zum Erlöskonto |
| Aufwandssammelkonto | Gegenkonto zum Aufwandskonto |

</details>

Die Bestandsbewertungskonten werden auf der Maske nur angezeigt, wenn der zugehörige Einrichterparameter auf **Ja** steht.

Diese Konten werden für Buchungen von Werten aus der [permanenten Inventur](../../../abschluesse_inventur/permanente_inventur/fibu_buchung_der_permanenten_inventuren.md) verwendet.

| Bestandsbewertungskonten |
| --- |
| Zugangskonto | Konto für die Buchung des SOLL-Bestandes |
| Abgangskonto | Konto für die Buchung des IST-Bestandes |
| Inventurkonto | Konto für die Buchung der Bestandsdifferenz |

<p class="just-emphasize">Einrichterparameter</p>

| Einrichterparameter | Beschreibung | Vorbelegung |
| --- | --- | --- |
| Sollen die Bestandsbewertungskonten auch angezeigt werden? | Bei ‚Ja‘ werden die Felder auf der Maske eingeblendet. | Nein |

<p class="siehe-auch">Siehe auch:</p>

- [Erlöskennziffer Kontozuordnung bei Steuersatzänderung](./erloeskennziffer_kontozuordnung_bei_steuersatzaenderung.md)
