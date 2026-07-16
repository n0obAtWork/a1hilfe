# Stammdaten Steuersätze

<!-- source: https://amic.de/hilfe/stammdatensteuerstze.htm -->

Hauptmenü > Abschlussarbeiten > Umsatzsteuer > Steuern

Direktsprung **[STS]**.

Die Pflege der Steuersätze kann an zwei unterschiedlichen Stellen erfolgen. Ruft man den Steuersatzpfleger auf, so erreicht man einen Kreuzpfleger, der die einzelnen Bestandteile eines Satzes gruppiert ausgibt und so versucht, eine übersichtlichere Darstellung zu bieten. Durch Anklicken der entsprechenden Spalte kann man die dazugehörigen Daten pflegen. Es existiert dazu noch ein Pfleger, der über die bekannte Auswahllistenmechanik die Daten anzeigt. Dieser ist von hier aus über das Menü zu erreichen (***Steuersätze* F8**).

Das System bestimmt den passenden Satz über eine Kombination von 4 Elementen:

- Steuerklasse. Umsatzsteuer oder Vorsteuer, Brutto oder Netto.
- Steuergruppe. Inlandskunde, Auslandskunde...
- Steuerschlüssel. Steuerfrei, Voller Steuersatz, verminderter Steuersatz...
- Steuerabdatum. Ab und an ändert sich der Steuersatz. Letztes Beispiel war die Erhöhung des vollen Steuersatzes auf 19% zum 01.01.2007. Dann ist es nur nötig, für das Änderungsdatum einen neuen Satz zu hinterlegen, damit das System weiß, welcher Steuersatz gültig ist. Dazu gibt es im Kreuzpfleger eine Funktion "***Speichern unter* Shift+F9**"

Zu den oben genannten Kombinationen müssen jetzt noch weitere Daten hinterlegt werden.

![](../../../ImagesExt/image8_564.png)

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td></td>
        <td>
          <p><strong>Beschreibung</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Steuerformel</p>
        </td>
        <td>
          <p>Hier gibt es vier Möglichkeiten:<br><br><u>Normale Steuer</u>. Hier wird die Steuer nach der gebräuchlichen Formel für Vorsteuer bzw. Umsatzsteuer berechnet.</p>
          <p><u>Steuer 100%.</u> Mit dieser Einstellung können Steuerkonten in der Finanzbuchhaltung direkt bebucht werden. Es bedeutet, dass der gesamte eingegebene Betrag dem Steuerkonto zugeordnet wird. Siehe dazu "Steuerkonten bebuchen"</p>
          <p><u>Reisekosten</u>: Die Angabe der Steuersätze für Reisekosten erfolgt "in Hundert". Das bedeutet, dass nicht die normale Steuerformel verwendet werden kann.</p>
          <p><u>Innergemeinschaftlicher Erwerb: </u>Wird ein Beleg mit einem Steuersatz erfasst, der diese Steuerformel beinhaltet, so wird bei der Erfassung keine Steuerzeile erzeugt. Beim Buchen werden, wenn die Konten für den innergemeinschaftlichen Erwerb angegeben wurden, zwei Steuerzeilen dem Beleg hinzugefügt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Steuersatz</p>
        </td>
        <td>
          <p>Hier wird der Steuersatz eingetragen, z.B. 19,00.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Steuerkonto</p>
        </td>
        <td>
          <p>Das zugehörige Steuerkonto in der Finanzbuchhaltung, auf dem die bei der Fakturierung oder in der Buchhaltung ermittelten Beträge verbucht werden sollen. Vor der Anlage von Steuersätzen muss also ein Steuerkonto im Sachkontenstamm erfasst werden. Es ist nur der Eintrag von Konten möglich, bei denen das Kennzeichen "Steuerkonto" auf <b>Ja</b> steht.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Skontokonto</p>
        </td>
        <td>
          <p>Das dem Steuerkonto zugeordnete Skontokonto, auf dem die bei der Fakturierung oder in der Buchhaltung ermittelten Skontobeträge verbucht werden sollen. Vor der Anlage von Steuersätzen muss also auch ein Skontokonto erfasst werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Steuersatz</p>
        </td>
        <td>
          <p>Hier wird der Steuersatz eingetragen, z.B. 19,00.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Steuerkonto</p>
        </td>
        <td>
          <p>Das zugehörige Steuerkonto in der Finanzbuchhaltung, auf dem die bei der Fakturierung oder in der Buchhaltung ermittelten Beträge verbucht werden sollen. Vor der Anlage von Steuersätzen muss also ein Steuerkonto im Sachkontenstamm erfasst werden. Es ist nur der Eintrag von Konten möglich, bei denen das Kennzeichen "<b>Steuerkonto</b>" auf <b>Ja</b> steht.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Skontokonto</p>
        </td>
        <td>
          <p>Das dem Steuerkonto zugeordnete Skontokonto, auf dem die bei der Fakturierung oder in der Buchhaltung ermittelten Skontobeträge verbucht werden sollen. Vor der Anlage von Steuersätzen muss also auch ein Skontokonto erfasst werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Satz innergem.Erw.</p>
        </td>
        <td>
          <p>Der für "Innergemeinschaftliche Erwerbe" zu verwendende Steuersatz</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Konto innergem.Erw.MwSt.</p>
        </td>
        <td>
          <p>Der Umsatzsteueranteil "Innergemeinschaftliche Erwerbe" wird diesem Konto zugeordnet</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Konto innergem.Erw.VSt.</p>
        </td>
        <td>
          <p>Der Vorsteueranteil "Innergemeinschaftliche Erwerbe" wird diesem Konto zugeordnet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Text</p>
        </td>
        <td>
          <p>Dieser Text kann beim Formulardruck in der Warenwirtschaft mit angedruckt werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Fremdschlüssel</p>
        </td>
        <td>
          <p>Hier kann man für den Steuersatz ein Kürzel angeben, welches ihn lesbar macht. Z.B.“VB19“ für „Vorsteuer Brutto 19%“. Diese Kürzel kann auf dem Kontoblatt (Formulardruck) als „Steuersatzfremd“ gedruckt werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>DATEV-Schlüssel</p>
        </td>
        <td>
          <p>Wenn man die Daten zur DATEV übertragen will, wird hier der von der DATEV verwendetet Steuerschlüssel hinterlegt. Es ist zum einen dann notwendig, wenn der Einrichtungsparameter „Datevschlüssel im Textfeld übergeben“ auf <b>Ja</b> steht, oder dann, wenn man im <a href="../../fibu_schnittstellen/exportverfahren_der_finanzbuchhaltung/datev/datev_firmenstamm.md">DATEV-Firmenstamm</a> als Übertragungs-Format „Übertragung des Umsatzsteuerschlüssels“ gewählt hat. Für die Steuerformel „Steuer 100%“ muss der DATEV-Schlüssel leer bleibe. Mögliche Schlüssel lassen sich mit <strong>F3</strong> auswählen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Exportschlüssel</p>
        </td>
        <td>
          <p>Wenn man Belege aus der Finanzbuchhaltung in ein Fremdsystem exportieren möchte, kann es nötig sein, die Kennzeichnung des Steuerschlüssels der Fremdfibu zu hinterlegen. Beim Export in das IBM-Finanzwesen (siehe EXPORT) wird dieser Schlüssel verwendet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Aw-Kennz Umsatz</p>
        </td>
        <td>
          <p>Hier muss die Auswertungsposition für die Auswertung nach Auswertungspositionen&nbsp;bzw. für den Ausdruck über das Umsatzsteuer-Voranmeldungsformular eingerichtet werden. Für alle Steuersätze, deren <b>Bemessungsgrundlage</b> auf dem Umsatzsteuerformular eingetragen werden muss, müssen hier die entsprechenden Zeilen hinterlegt werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Aw-Kennz Steuer</p>
        </td>
        <td>
          <p>Hier muss die Auswertungsposition für die Auswertung nach Auswertungspositionen&nbsp;bzw. für den Ausdruck über das Umsatzsteuer-Voranmeldungsformular eingetragen werden. Für alle Steuersätze, deren <b>Steuerbetrag</b> auf dem Umsatzsteuerformular&nbsp;eingetragen werden muss, muss hier die entsprechende Zeile hinterlegt werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Aw-Kennz Einfuhrsteuer (MwSt)</p>
        </td>
        <td>
          <p>Innergemeinschaftliche Erwerbe unterliegen bekanntlich einer besonderen Ausweispflicht auf dem Umsatzsteuerformular.</p>
          <p>Weist die anfallende Erwerbssteuer anhand dem unter „<b>Satz innergem.Erw.</b>“ hinterlegten Steuersatz aus.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Aw-Kennz Einfuhrsteuer (VSt)</p>
        </td>
        <td>
          <p>Zieht die Erwerbssteuer in Zeile 56 / Kennziffer 61 anhand dem unter „Satz innergem.Erw.“ hinterlegten Steuersatz wieder ab.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Aw-Kennz Minderung Bemessungsgrundlage</p>
        </td>
        <td rowspan="2">
          <p>Seit 2021: <a href="./kennziffern_fuer_ergaenzende_angaben.md">Ergänzende Angaben</a> zu Minderungen nach § 17 Abs. 1 Sätze 1 und 2 i.V.m. Abs. 2 Nr. 1 Satz 1 UstG.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Aw-Kennz Minderung abziehbare Vorsteuer</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>eRechnung Steuerkategorie</p>
        </td>
        <td>
          <p>Kategorie der Steuer für die Ausgabe in der eRechnung. In der Regel wird dies „S“-Standard sein, jedoch sind hier Steuerbefreiungen, oder spezielle Steuern zu pflegen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ausnahmecode</p>
        </td>
        <td>
          <p>Wenn die Steuerkategorie eine Ermäßigung oder Steuerbefreiung enthält, muss eine Begründung in Form eines Ausnahmecodes angegeben werden. Diese Ausnahmen sind in den <a href="https://www.xrepository.de/details/urn:xoev-de:kosit:codeliste:vatex_1">VATEX</a> beschrieben.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

Die Funktion „***Steuersatz sperren“* SF7** sperrt den Steuersatz, der unter der Kombination aus Klasse, Gruppe und Schlüssel ausgewählt wurde. Wenn man die Funktion auswählt, öffnet sich eine Maske zur Erfassung des Datums. Dort gibt man das Datum an, ab dem der Steuersatz gesperrt werden soll. Belege, die nach diesem Datum erfasst werden, können dann diesen Steuersatz nicht mehr verwenden.

Die Funktion „**<em>Speichern Unter“</em> Shift+F9** dient normalerweise dazu, denselben Steuersatz unter einem anderen Datum abzuspeichern. Daher wir nur das Steuerabdatum freigegeben. Mit dem Einrichterparameter „Bei „Speichern unter“ alle Schlüsselfelder freigeben“ wird diese Funktionalität aufgehoben und man kann auch Steuerklasse, Steuerschlüssel und Steuergruppe ändern.
