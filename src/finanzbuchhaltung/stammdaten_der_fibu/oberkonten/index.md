# Oberkonten

<!-- source: https://amic.de/hilfe/oberkonten.htm -->

Hauptmenü > Finanzbuchhaltung > Stammdaten > Oberkonten

Direktsprung **[OKS]**

Bei Oberkonten handelt es sich um Konten, die ausschließlich aus Informationsgründen angelegt werden. Obwohl sie Bestandteil des Kontenplans sind, können Oberkonten natürlich nicht direkt bebucht werden; auf ihnen werden nur Daten zusammengefasst. Mit ihrer Hilfe werden die Stände der direkt bebuchbaren Sachkonten schrittweise zu aussagekräftigeren Gesamtgrößen zusammengezogen. So kann man z.B. die Ergebnisse der Konten **"Erlöse Agrar", "Erlöse Baustoffe", "Erlöse Mineralöl"** zum Oberkonto **"Erlöse Gesamt"** zusammenziehen.

In dem Erfassungsbildschirm können die nachfolgenden Felder bearbeitet werden:

| | Beschreibung |
| --- | --- |
| Nummer | Identifikation des Oberkontos.  
In der Finanzbuchhaltung ist es Sinnvoll, die Nummern für Sach-, Personen- und Oberkonten in getrennten Bereichen zu haben. Diese Bereiche können in A.eins selber festgelegt werden. Dies geschieht über Nummernkreise und deren Ober- und Untergrenzen. Die Nummernkreise für die Kontogruppen werden in der „**Allgemeinen Nummernkreiszuordnung**“ (Direktsprung **[MNDNK])** festgelegt.  
Ist kein Nummernkreis in der **Allgemeinen Nummernkreiszuordnung**“ festgelegt, können keine Oberkonten erfasst werden. Dieses Verhalten lässt sich per Einrichterparameter „**Nummernkreiszuordnung ignorieren**“ ändern, indem man den Wert auf **Ja** ändert. Es findet dann kein Bereichstest statt.  
 |
| Matchcode | Matchcode für das Oberkonto.  
 |
| Bezeichnung | Bezeichnung des Oberkontos für Auswahllisten etc.  
Ist der Steuerungsparameter 34 "Mehrsprachigkeit aktiv“ in A.eins gesetzt, so hat man auf diesem Feld die Möglichkeit mit F3 [sprachabhängige Bezeichnungen](../../../firmenstamm/a_eins_sprache/sprachabhaengige_bezeichnung_in_den_stammdaten.md) zu pflegen.  
 |
| Kontotyp | Hier wird der Kontentyp festgelegt:  
0 **= Bilanzkonto**  
**1 = GuV**  
**3 = Statistik**  
Hinter dem Kontotyp kann ein Oberkonto noch – bei Bilanzkonten – als Aktiv oder Passivkonto oder – bei GuV-Konten - als Aufwands oder Ertragskonto. gekennzeichnet werden |
| **Oberkonto  
    
** | Ein Oberkonto **KANN** wiederum selbst einem Oberkonto zugeordnet sein - dann macht die Verteilung natürlich keinen Sinn! So können mehrstufige Oberkonten-Summierungen erzeugt werden. Hierdurch kann eine hierarchische Verdichtung des Kontenplans erfolgen.  
Es ist zurzeit eine maximale Verschachtelungstiefe von bis zu 8 Stufen zugelassen.  
 |
| **Verteilung** | Für spezielle Auswertungsansprüche können die auf einem Oberkonto auflaufenden Beträge nach einem einstellbaren Schema weiterverteilt werden. Üblicherweise wird allerdings - und so ist auch die Voreinstellung - keine Verteilung erfolgen.  
Wenn man hier **Ja** einträgt, wird das Konto als Verteilkonto behandelt, bei dem selbst keine Summen auflaufen, sondern das nur der Verteilung der "eingehenden" Beträge auf mehrere andere Oberkonten dient. Die Verteilung kann dann auf dem Register „Verteilung“ eingegebene werden. Dieses Register erscheint nur bei Verteilkonten.  
 |
| **100%** | Nur bei Verteilkonten: Kennzeichen, ob die Prozente genau auf hundert aufgehen müssen. Anderenfalls wird u. U. mehr oder weniger auf die Oberkonten "verteilt", als eigentlich zu verteilen wäre - was aber durchaus sinnvoll sein kann!  
 |
| **Druckposition Bilanz / GuV Soll/Haben** | **Da** **es Konten** – z.B. Bankkonten—gibt, die je nach ihrem Saldo einmal auf der Passiv und einmal auf der Aktivseite der Bilanz erscheinen können, kann man hier getrennt nach Soll oder Haben die Druckposition erfassen. Ist diese Trennung nicht notwendig, so trägt man einfach in beide Eingabefelder die gleich Druckposition ein.  
 |
| Druckposition IFRS-Bilanz/GuV Soll/Haben | Es können parallel zur Bilanz nach HGB auch Druckpositionen für eine IFRS – Bilanz hinterlegt werden.  
 |
| Druckpositionen US-GAAP-Bilanz**/GuV Soll/Haben  
    
** | Die **U**nited **S**tates **G**enerally **A**ccepted **A**ccounting **P**rinciples (US-GAAP) ist die allgemeine Bezeichnung für die US-amerikanischen Vorschriften der Rechnungslegung, die die Buchführung sowie den Jahresabschluss der Unternehmen regeln. Innerhalb der USA spricht man nur von GAAP, entsprechend etwa dem deutschen Sprachgebrauch den GOB’s. Um zusätzlich zu der Steuerbilanz auf HGB – Basis eine Bilanz auf US-GAAP-Basis erstellen zu können, existieren im Sachkontenstamm diese Druckpositionen.  
 |
| **Druckposition Summen & Saldenliste** | Druckposition in Summen- und Saldenliste.  
 |
| **BWA 100% Bezug** | In den BWA Listen "BWA Jahresvergleich" und "BWA Monatsvergleich" wird eine prozentuelle Abweichung zu diesem Oberkonto errechnet. Es darf und muss nur ein Oberkonto mit **Ja** gekennzeichnet sein.  
 |
| **Farbe Planabweichung > 0% bzw. &lt; 0%** | Die prozentuelle Abweichung von den Planzahlen lässt sich farblich hervorheben, wenn man über oder unter Plan liegt.  
 |
| Auswertungsposition A-C | In den drei Feldern **"Auswertungsposition"** können frei vorgebbare Selektionsmerkmale eingetragen werden. Auf sie kann für Auswertungszwecke zugegriffen werden. Über **F3** kann die Liste der (vom Anwender) voreingestellten Auswertungspositionen abgerufen werden.  
    
 |
| Kennzahl für externe Auswertungen | Weitere frei definierbare Kennzahl bzw. Auswertungsposition. Sie wird von A.eins nicht verwendet. Über **F3** kann auf die die Liste der (vom Anwender) voreingestellten Auswertungspositionen zugegriffen werden.  
    
 |

**Anmerkung:  
***Hier soll noch einmal ausdrücklich darauf hingewiesen werden, dass nicht umgebucht wird. Es werden lediglich zu Auswertungszwecken Daten **"verteilt"**. Die Buchhaltung wird also nicht beeinflusst. Um jedoch in den Auswertungen einen logischen Datenfluss sicherzustellen, sollte der Umgang mit dem Instrument **"Verteilung"** sorgfältig geplant werden.*

ADDON

Für Oberkonten können eigene Felder und somit eigene Daten erfasst werden. Die Definition geschieht über das [A.eins Informationssystem](../../../zusatzprogramme/ais_a_eins_informationssystem/index.md) (Direktsprung **[AIS]**). Hier kann man z.B. für die vordefinierte Tabelle OBERSACHKONTOADDON oder eigenen privaten Tabellen zusätzliche Felder definieren, die dann auf den definierten Registern erscheinen.

<p class="siehe-auch">Siehe auch:</p>

- [Report Oberkonten](./report_oberkonten.md)
