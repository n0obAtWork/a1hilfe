# Funktionen des VIMP-Pflegers

<!-- source: https://amic.de/hilfe/_vimp_plausibilitaet.htm -->

In der Optionbox der Masken und der Auswahlliste existieren folgende Funktionen:

<details>
<summary>Funktionen des VIMP-Pflegers</summary>

| Funktion | Beschreibung |
| --- | --- |
| Status zurücksetzen. **(STRG+F5)** | Mit dieser Funktion wird auf der Auswahlliste der markierte Datensatz, wenn der Status 3 bis 7 oder 9 ist, auf 2 zurückgesetzt. Bei Problemen werden diese im Fehlerprotokoll angezeigt. |
| Datensatz als gelöscht markieren. **(STRG+F7)** | Mit dieser Funktion wird der Status auf 9 gesetzt. |
| Löschen **(F7)** | Öffnet die VIMP-Pfleger-Maske für den markierten Datensatz im Löschmodus. Es kann nur gelöscht werden, wenn der Status vorher auf 9 gesetzt wurde. |
| Standardvorgang erzeugen | Es wird aus den Daten des markierten Vorgangsimportes ein Vorgang erzeugt. Bei Problemen werden diese im Fehlerprotokoll angezeigt. |

Für die Funktion Status zurücksetzen gibt es einen Sonderfall. Vorgangsimport mit Vorgangsklasse 500(Ladeschein) und 1500(Eingangsladschein) werden mit der Funktion immer auf Status 5 gesetzt.

###### Standardvorgang erzeugen

<p class="just-emphasize">Allgemein</p>

Mit dieser Funktion kann ein Vorgang aus den Importieren Daten erzeugt werden. Es müssen die Positionen in der Auswahlliste markiert werden, aus denen dann ein Vorgang erzeugt werden soll und diese dürfen keine rotmarkierten Felder mehr in der Auswahlliste besitzen. 

Kann ein Vorgang bei der Vorgangserzeugung nicht angelegt werden, so wird der Status für den Stammsatz und allen dazugehörigen Positionen auf „Fehlerhaft“ gesetzt.

Kann eine Position bei der Vorgangserzeugung im Vorgang nicht angelegt werden, so wird der Status für diese Position auf Fehlerhaft gesetzt. Ansonsten wird nach erfolgreicher Erstellung des Vorgangs der Status für beide Kennzeichen auf „Erledigt“ gesetzt. Des Weiteren wird die Vorgangsnummer und die Vorgangsid in des Stammsatz geschrieben, so hat man den Überblick darüber, welcher Vorgang aus diesem Satz erzeugt worden ist.

<p class="just-emphasize">Folgende Felder werden in den Vorgang übernommen.</p>

<p class="just-emphasize">Relation [ImportVorgStamm](http://www.amic.de/ihilfe/XMLDocuments/iAeins/html/T_Datenmodell_ImportVorgstamm.htm)</p>

In dieser Relation werden Kopfdaten des Vorgangs hinterlegt.

| Felderbeschreibung | ID’s Info | Datenbankfeld |
| --- | --- | --- |
| Kundenummer | Kundummer wird bei der Vorgangsanlegung gesetzt | KundNummer |
| Vorgangsnummer | NumNummer bei der Vorgangsanlegung. Die NumNummer wird nur gesetzt, wenn in diesem Feld ein Wert > 0 gestzt worden ist. Es erfolgt keine Prüfung ob die Vorgangsnummer in Kombination mit der Vorgangsklasse, Vorgansgunterklasse sowie der Jahrnummer schon vorhanden ist | V_NumNummer |
| Vorgangsklasse | Klasse wird bei der Vorgangsanlegung gesetzt | V_KlassNummer |
| Vorgangsunterklasse | UnterKlasse wird bei der Vorgangsanlegung gesetzt | V_UKlassNUmmer |

<p class="just-emphasize">Relation [ImportVorgPosition](http://www.amic.de/ihilfe/XMLDocuments/iAeins/html/T_Datenmodell_ImportVorgPosition.htm)</p>

In dieser Relation werden Daten der Vorgangswarenposition gespeichert.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><strong>Felder</strong></p>
        </td>
        <td>
          <p><strong>ID’s / Infos</strong></p>
        </td>
        <td>
          <p><strong>Datenbankfeld</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Artikel</p>
        </td>
        <td>
          <p>Artikelid bei der Positionsanlage</p>
        </td>
        <td>
          <p>ArtikelId</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Variante</p>
        </td>
        <td>
          <p>Variante bei der Positonsanlage</p>
        </td>
        <td>
          <p>ArtikelVariante</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Partie</p>
        </td>
        <td>
          <p>ID_PARTIENUMMER</p>
        </td>
        <td>
          <p>Partienummer</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Lagernummer</p>
        </td>
        <td>
          <p>ID_LAGERNUMMER</p>
        </td>
        <td>
          <p>Lagernummer</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Lagerplatz</p>
        </td>
        <td>
          <p>ID_LAGERPLATZ</p>
        </td>
        <td>
          <p>LagerPlatzNummer</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>ME_Nummer</p>
        </td>
        <td>
          <p>ID_ME_NUMMER</p>
        </td>
        <td>
          <p>ME</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>ME_NummerPreis</p>
        </td>
        <td>
          <p>ID_ME_NUMMER_PREIS</p>
        </td>
        <td>
          <p>ME_Preis</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Preis</p>
        </td>
        <td>
          <p>ID_PREIS</p>
        </td>
        <td>
          <p>Preis</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Preiseinheit</p>
        </td>
        <td>
          <p>ID_PREISEINHEIT</p>
        </td>
        <td>
          <p>Preiseinheit</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kontrakt</p>
        </td>
        <td>
          <p>ID_KONTRAKT_NUMMER</p>
        </td>
        <td>
          <p>KonraktNummer</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Zusatzinfo</p>
        </td>
        <td>
          <p>ID_ZUSATZINFO</p>
        </td>
        <td>
          <p>ZusatzInfo</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Zusatinfo2</p>
        </td>
        <td>
          <p>ID_ZUSATZINFO2</p>
        </td>
        <td>
          <p>ZusatzInfo2</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Gebinde Informationen</p>
          <table>
            <tbody>
              <tr>
                <th>Felder</th>
                <th>ID‘s</th>
              </tr>
              <tr>
                <td>Anzahl</td>
                <td>ID_GEBINDE</td>
              </tr>
              <tr>
                <td>Gebindefaktor1</td>
                <td>ID_GEBINDEMASS_1</td>
              </tr>
              <tr>
                <td>Gebindefaktor2</td>
                <td>ID_GEBINDEMASS_2</td>
              </tr>
              <tr>
                <td>Gebindefaktor3</td>
                <td>ID_GEBINDEMASS_3</td>
              </tr>
              <tr>
                <td>Gebindefaktor4</td>
                <td>ID_GEBINDEMASS_4</td>
              </tr>
            </tbody>
          </table>
        </td>
        <td>
          <p>Diese werden per Datenbankprozedur Prozedur „<a href="http://www.amic.de/ihilfe/XMLDocuments/iAeins/html/M_SQL_FUNCTION_Gebinde_12_8bab589d.htm">Gebinde</a>“ bestimmt.</p>
        </td>
        <td>
          <p>GebFaktor1</p>
          <p>GebFaktor2</p>
          <p>GebFaktor3</p>
          <p>GebFaktor4</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Positionstexte Zeilenweise</p>
        </td>
        <td>
          <p>Per Funktion „Textneu“ und Zuordnung von ID_TEXTTEXT und der Zeilennummer.</p>
        </td>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>

<p class="just-emphasize">Relation ImportVorgPositionLVS</p>

Diese Relation beherbergt Informationen zu LVS-Ladeträgern, die zu dieser Position gehören.

| Felder | ID’s / Infos | Datenbankfeld |
| --- | --- | --- |
| UebernahmeID | Uebernahmeid der zugehörigen Position der Relation ImportVorgPosition | UebernahmeID |
| SatzID | SatzId der zugehörigen Position der Relation ImportVorgPosition | SatzID |
| PositionID | Positions-ID der zugehörigen Position der Relation ImportVorgPosition | PositionID |
| PositionZaehler | Laufender Zähler der LVS-Informationen zu der gegebenen Position | PositionZaehler |
| LokalitaetsNr | Nummer des Ladeträgerstandorts | LokalitaetsNr |
| LadetraegerNr | Nummer des Ladeträgers | LadetraegerNr |
| LadeeinheitsNr | Nummer der Ladeeinheit | LadeeinheitsNr |
| LadeeinheitsPosition | Nummer der Ladeeinheitsposition auf dem Ladeträger | LadeeinheitsPosition |
| BewegungsId | | BewegungsId |
| LadetraegerExtNummer | Externe Nummer des Ladeträgers (z.B. eine NVE) | LadetraegerExtNummer |
| Menge | Menge auf dem Ladeträger | Menge |
| ME_Nummer | Mengeneinheit der Menge auf dem Ladeträger | ME_Nummer |
| IVP_GUID | Guid der dazugehörigen Position der Relation<br>ImportVorgPosition | IVP_GUID |

<p class="just-emphasize">Relation ImportVorgPositionPartie</p>

In dieser Relation werden Informationen der Partie(n) einer Position abgelegt. Eine Partie, die hier eingetragen ist, jedoch im System noch nicht existiert, wird angelegt werden.

| Felder | ID’s / Infos | Datenbankfeld |
| --- | --- | --- |
| IVP_GUID | Guid der dazugehörigen Position der Relation<br>ImportVorgPosition | |
| Zaehler | Partiezähler | |
| PartieId | PartieId | |
| PartieNummer | Partienummer<br>Ist die Partienummer gesetzt und die Partiebezeichnung wird mit der Kombination<br>Partienummer und Partiebezeichnung nach der Partie gesucht. Wenn nur die Partienummer gesetzt worden ist wird nach der Partienummer gesucht<br>Existiert mehr als eine Partie zu einer Partienummer wird immer die erste Partie gewählt | |
| PartieBezeichnung | Ist nur die Partiebezeichnung angegeben worden, und zu dieser Partie wurde keine aktive Partie gefunden, so wird eine neue Partie angelegt.<br>Sind Partienummer und Partiebezeichnung angegeben, so wird die Partie nach dieser Kombination gesucht. | |
| PartieAbDatum | Wird bei Neuanlage einer Partie ausgewertet und als Partieabdatum gesetzt | |
| PartieBisDatum | Wird bei Neuanlage einer Partie ausgewertet und als Partiebisdatum gesetzt | |
| Menge | Menge der Partie | |
| ME | Mengeneinheit der Partie | |

<p class="just-emphasize">Relation ImportVorgScannung</p>

| Felder | ID’s / Infos | Datenbankfeld |
| --- | --- | --- |
| UebernahmeID | | |
| SatzID | | |
| PositionID | | |
| PositionZaehler | | |
| AiListe | | |
| SatzTyp | | |
| Scannung | | |
| MarkierIdent | | |

<p class="just-emphasize">Relation ImportVorgStammAddOn</p>

Aus dieser Relation werden Vorgangsaddon-Felder des Vorgangs befüllt. Der AddOnName muss dem des Feldes in der Tabelle VorgangAddOn entsprechen.

| Felder | ID’s / Infos | Datenbankfeld |
| --- | --- | --- |
| IVS_GUID | GUID des korrespondierenden Eintrags in der Relation ImportVorgStamm | |
| AddonName | Name des AddOnFeldes | |
| AddonWert | Wert des AddOnFeldes | |

<p class="just-emphasize">Relation ImportVorgStammUFLD</p>

In dieser Relation werden die Setzungen von UFLD-Feldern für den Vorgang vorgenommen. Bitte beachten Sie, dass nur UFLD-Felder gesetzt werden können, die vom importierenden Bediener für den jeweiligen Vorgang gesetzt werden dürfen.

| Felder | ID’s / Infos | Datenbankfeld |
| --- | --- | --- |
| IVS_GUID | GUID des korrespondierenden Eintrags in der Relation ImportVorgStamm | |
| UFLDId | ID des UFLD-Feldes | |
| UFLDWert | Wert des UFLD-Feldes | |

<p class="just-emphasize">Relation ImportVorgTextPosition</p>

In dieser Relation werden Textpositionen hinterlegt, die entweder vor oder nach einer Position in den Beleg eingefügt werden können.

| Felder | ID’s / Infos | Datenbankfeld |
| --- | --- | --- |
| UebernahmeID | Uebernahmeid der zugehörigen Position der Relation ImportVorgPosition | UebernahmeID |
| SatzID | SatzId der zugehörigen Position der Relation ImportVorgPosition | SatzID |
| PositionID | Positions-ID der zugehörigen Position der Relation ImportVorgPosition | PositionID |
| ZeilenZaehler | Laufende Nummer der Textposition | ZeilenZaehler |
| TextTyp | 0 = Positionstext | TextTyp |
| TextPosition | 0 = Vor der Position anzeigen<br>1 = Nach der Position anzeigen | TextPosition |
| VorgText | Text der Positionszeile | VorgText |
| IVP_GUID | Guid der dazugehörigen Position der Relation<br>ImportVorgPosition | IVP_GUID |

<p class="just-emphasize">Relation ImportVorgStammZusatzTexte</p>

| Felder | ID’s / Infos | Datenbankfeld |
| --- | --- | --- |
| IVS_GUID | GUID des korrespondierenden Eintrags in der Relation ImportVorgStamm | |
| TextTyp | Typ der Textzeile (Header/Line) | |
| LineNo | Zeilennummer | |
| TextZeile | Text dieser Zeile | |

<p class="just-emphasize">Relation ImportVorgStammZuAb</p>

Diese Relation beinhaltet einen Verweis auf die zu diesem Beleg anzuwendenden Zu-Abschläge/Frachten oder Rabatte. Da im Vorgangsimport keine Gruppen definiert werden können, wird dieser Zu/Abschlag stets auf die Gruppe 0 angewendet.

| *Felder* | *ID’s / Infos* | *Datenbankfeld* |
| --- | --- | --- |
| IVS_GUID | Guid der ImportVorgStamm | |
| IVZ_Zaehler | Laufender Zähler des Zu/Abschlags | |
| IVZ_GUID | Guid der Definition | |

<p class="just-emphasize">Relation ImportVorgPositionZuAb</p>

Diese Relation beinhaltet einen Verweis auf die zu dieser Position anzuwendenden Zu-Abschläge/Frachten oder Rabatte

| Felder | ID’s / Infos | Datenbankfeld |
| --- | --- | --- |
| IVP_GUID | Guid der ImportVorgPosition | |
| IVZ_Zaehler | Laufender Zähler des Zu/Abschlags | |
| IVZ_GUID | Guid der Definition | |

<p class="just-emphasize">Relation [ImportVorgZuAbDef](../tabellen_des_vorgangsimports/importvorgzuabdef.md)</p>

Diese Relation beinhaltet die Definition eines Zu/Abschlags, eines Rabattes oder einer Fracht.

Bitte beachten Sie, dass das Setzen einiger Werte abhängig von der verwendeten Berechnungsformel ist. Das Setzen eines nicht relevanten Wertes kann ggf. zum Abbruch und wird in jedem Fall jedoch zu einem Eintrag ins Fehlerprotokoll führen.

###### Positionen bearbeiten

<p class="just-emphasize">Allgemein</p>

In dieser Maske können Änderungen an einer vorhandenen Position durchgeführt werden.

Bei der Änderung des Kunden / Lieferanten oder der Vorgangsunterklasse wird die zu gerade bearbeitende Position als gelöscht markiert. Es wird ein neuer Stammsatz mit dem Kunden und der Unterklasse angelegt. Existiert für den Kunden/Lieferanten und der Unterklasse noch ein nicht weiterverarbeiteter Beleg, so wird diese Position an diesen Beleg angehängt.

Hierbei ist zu beachten, dass wenn mehrere Positionen markiert wurden und beim Blättern wird bei einem Datensatz der Lieferant oder die Unterklasse gewechselt, so wird beim Zurückblättern der geänderte Lieferant oder die geänderte Unterklasse nicht angezeigt, da die Kopfdaten der Aktuellen Position auf der Maske dargestellt werden.

Bei allen anderen Änderungen werden die Daten an der aktuellen Position abgeändert.

<p class="just-emphasize">Partie-Neuanlage</p>

Mit dieser Funktion kann eine neue Partie für den Artikel in dieser Position angelegt werden. Diese Partie wird dann Automatisch in das Partiefeld übernommen.

<p class="just-emphasize">Positionstext</p>

Auf dieser Registerkarte kann ein Positionstext eingetragen werden, welcher am Ende der Positionszeile eingefügt wird. Der Text darf nur 100 Zeichen pro Textzeile haben. Es sind aber mehrere Textzeilen möglich.

###### Plausbibilitätsprüfung des Vorgangsimportes

Um die Plausibilität von einem Vorgangsimport zu überprüfen, muss man auf die Variante Vorgangimport wechseln.

Auf jeder VIMP-Pfleger-Maske existiert die Funktion Plausibilität, die man mit F4 aufrufen kann.

Diese öffnet eine neue Maske und zeigt dort jeweils von der aktuellen Ebene abwärts Hinweise, Warnungen und Fehler im Import an.

Zuerst landet man auf die Maske ImportVorgStamm auf der alle Daten aus der Tabelle ImportVorgstamm enthalten sind.

Auf den Tabreiter Stammdaten und Zusätzliche Daten sind die für den Vorgangsimport relevanten Daten, die teilweise im Ändernmodus pflegbar sind. Diese lassen sich nur pflegen, wenn der Status des Vorgangsimportes 2 ist,

was auf der Maske als bereit(2) angezeigt wird.

Es sind nur die Felder Unterklasse und Kunde auf dem Tabreiter Stammdaten nachträglich veränderbar.

Auf den Tabreiter Fibu-Daten und Ungenutzte Felder befinden sich Felder, deren Inhalt nur informatorischen Inhalt besitzen und den Vorgangsimport nicht beeinflussen.

Auf dem Tabreiter Addon ist eine Tabelle mit dem Inhalt der Tabelle ImportVorgStammAddon.

Auf dem Tabreiter UFLD ist eine Tabelle mit dem Inhalt der Tabelle ImportVorgStammUFLD.

Auf dem Tabreiter Zusatztexte ist eine Tabelle mit dem Inhalt der Tabelle ImportVorgStammZusatztexte.

Von der Maske Importvorgstamm können drei weitere Maske geöffnet werden.

Durch das Drücken von F4 kann die Plausibilitätsmaske geöffnet werden, die Ereignisse, Warnungen und Fehler in einer Tabelle der öffnenden Maske anzeigt.

Auf dem Tabreiter ZuAbschläge ist eine Tabelle mit dem Inhalt der Tabelle ImportVorgStammZuAb. Durch das Doppelklicken auf einen Eintrag in der Spalte Zähler öffnet sich die Maske ImportVorgZuAb.

Auf dem Tabreiter Position ist eine Tabelle mit dem Inhalt der Tabelle ImportVorgPosition. Durch das Doppelklicken auf einen Eintrag in der Spalte PositionId öffnet sich die Maske ImportVorgPosition.

Die Maske ImportVorgZuAb dient nur zu Informationszwecken und besitzt eine durch F4 aufrufbare Plausibilitätsprüfung. Wenn diese Maske über die ImportVorgStamm Maske aufgerufen wurde, gibt es kein Feld für PositionId.

Wenn der Typ Platzhalte Fracht ist, dann sind auch drei Felder zur Fracht einsehbar.

Auf der Maske ImportVorgPosition sind alle Daten aus der Tabelle ImportVorgPosition.

Auf dem Tabreiter Stammdaten und Zusätzliche Daten sind die für den Vorgangsimport relevanten Daten, die teilweise im Änderungsmodus pflegbar sind. Diese lassen sich nur pflegen, wenn der Status des Vorgangsimportes 2 ist,

was auf der Maske als bereit(2) angezeigt wird.

Es sind nur die Felder Artikelnummer, Lagernummer, Menge, Mengeneinheit, Preis, Preiseinheit, Mindeshaltbarkeitsdatum(MHD), Partienummer und Lagerplatznummer auf dem Tabreiter Stammdaten nachträglich veränderbar.

Auf dem Tabreiter Zusätzliche Daten sind die Felder Kontraktnummer, NVE, Zusatzinfo und Zusatzinfo 2 nachträglich veränderbar.

Auf dem Tabreiter Ungenutzte Felder befinden sich Felder, deren Inhalt nur informatorischen Inhalt besitzen und den Vorgangsimport nicht beeinflussen.

Auf dem Tabreiter Partie ist eine Tabelle mit dem Inhalt der Tabelle ImportVorgPositionPartie.

Auf dem Tabreiter ZuAbschläge ist eine Tabelle mit dem Inhalt der Tabelle ImportVorgPositionZuAb.

Auf dem Tabreiter Addon ist eine Tabelle mit dem Inhalt der Tabelle ImportVorgPositionAddon.

Auf dem Tabreiter Textposition ist eine Tabelle mit dem Inhalt der Tabelle ImportVorgTextPosition.

Von der Maske ImportVorgPosition können drei weitere Masken geöffnet werden.

Durch das Drücken von F4 kann die Plausibilitätsmaske geöffnet werden, die Ereignisse, Warnungen und Fehler in einer Tabelle der öffnenden Maske anzeigt.

Auf dem Tabreiter LVS ist eine Tabelle mit dem Inhalt der Tabelle ImportVorgPositionLVS. Durch das Doppelklicken auf einen Eintrag in der Spalte Positionszähler öffnet sich die Mase ImportVorgPositionLVS.

Auf dem Tabreiter ZuAbschläge ist eine Tabelle mit dem Inhalt der Tabelle ImportVorgStammZuAb. Durch das Doppelklicken auf einen Eintrag in der Spalte Zähler öffnet sich die Maske ImportVorgZuAb.

Mit Hilfe des SPA 1131 kann man die Plausibilitätsprüfung auch vor dem Import mit useCS = 1 laufen lassen. Kommen dort Fehler auf, wird der Import auf fehlerhaft gestellt.

</details>
