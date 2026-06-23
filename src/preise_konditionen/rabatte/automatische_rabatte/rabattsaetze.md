# Rabattsätze

<!-- source: https://amic.de/hilfe/_Rabatte_RAS.htm -->

Preise / Konditionen > Rabatte > Rabattsätze

Oder Direktsprung **[RAS]**

Es gibt für Rabatte bestimmte Rabattsätze, die in bestimmten Zeiträumen gültig sind und in einer zu definierenden Formel zu berechnen sind.

<p class="just-emphasize">Rabattbezeichnung</p>

Bezeichnung für den Rabatt

<p class="just-emphasize">Steuerschlüssel</p>

Steuerschlüssel für diesen Rabatt

<p class="just-emphasize">Zu/Abschlagstyp (nur für [openTRANS](../../../firmenstamm/steuerparameter/lizenzen/opentrans_lizenz_spa_721.md))</p>

Hier wird die Art des Rabatts für openTRANS eingestellt. Dieser sollte als „Rabatt“ eingestellt werden. Andere Werte machen hier kaum Sinn.

<p class="just-emphasize">Ab Datum</p>

Beginn der Gültigkeit des Rabattsatzes

<p class="just-emphasize">Bis Datum</p>

Letzter Tag der Gültigkeit dieses Rabattsatzes

<p class="just-emphasize">Zu-/Abschlagsformel</p>

| Formel | Bedeutung |
| --- | --- |
| proz. v. Warenw. Abz. Vorh. Rabatte | Hier wird der Rabatt als prozentualer Wert des Warenwerts berechnet. Vorhandene Rabatte werden abgezogen, damit sich nicht beide Rabatte addieren |
| proz. v. reinen Warenwert | Wie oben, Vorhandene Rabatte werden jedoch NICHT abgezogen. Es kann zu einer Addition von Rabatten kommen. |
| proz. m. Preisrundung abz. Vorh. Rabatte | Hier wird der Rabatt als prozentualer Wert des Warenwerts berechnet.<br>Hier findet nach der Rabattberechnung eine Rundung der Einzelbeträge statt, so dass diese stimmig sind.<br>Vorhandene Rabatte werden abgezogen, damit sich nicht beide Rabatte addieren |
| proz. mit Preisrundung | Wie oben, Vorhandene Rabatte werden jedoch NICHT abgezogen. Es kann zu einer Addition von Rabatten kommen |
| Satz entspricht Einzelpreis | Es wird ein fester Rabatt auf die Warenposition gegeben. Z.B. 5€ für jeden Kauf eines Gerätes |
| Satz je Mengeneinheit | Es wird ein fester Rabattbetrag pro Mengeneinheit gegeben. Wird z.B. in Tonnen fakturiert, so wird der Rabatt pro Tonne berechnet. |
| Satz je Gebindeeinheit | Es wird ein fester Rabattbetrag pro Gebindeeinheit gegeben. Werden also z.B. 10 Paletten Steine gekauft, so wird 10x der Rabattbetrag berechnet. |
| Satz je Gewichtseinheit | Es wird ein fester Rabattbetrag pro Gewichtseinheit gegeben. Der Artikel wird in einer Mengeneinheit fakturiert, die nicht unbedingt das Gewicht beinhalten muss. Im Artikel kann jedoch ein Artikelgewicht hinterlegt werden.<br>So kann z.B. ein Einkauf von 10 Säcken (à 50kg) |
| Betrag manuell | Hier kann ein fester Betrag für einen Einkauf eingetragen werden. So kann z.B. stets 10€ für den Kauf mindestens eines bestimmten Artikels gegeben werden.<br>Die Einstellung „manuell“ bedeutet nicht, dass eine Rabattbetragseingabe in Verbindung mit der Warenposition erfolgen muss oder erzwungen wird oder werden kann. Sie ist vielmehr ein Kennzeichen, dass dieser Betrag nicht berechnet wurde. |

<p class="just-emphasize">Prozent/Preis</p>

Hier wird nun der Rabattbetrag (Satz) oder der Prozentwert (Prozent) eingetragen für den dieser Rabatt gelten soll.
