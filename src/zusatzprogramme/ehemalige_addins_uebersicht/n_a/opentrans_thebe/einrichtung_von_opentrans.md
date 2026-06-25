# Einrichtung von openTRANS

<!-- source: https://amic.de/hilfe/einrichtungvonopentrans.htm -->

Vor der Verwendung von openTRANS sind einige Dinge einzurichten, um einen einwandfreien Ablauf zu gewährleisten.

<p class="just-emphasize">Steuerparameter</p>

Der [Steuerparameter 721 – openTRANS](../../../../firmenstamm/steuerparameter/lizenzen/opentrans_lizenz_spa_721.md) (Lizenz) muss eingeschaltet sein, um die notwendigen Funktionen, Varianten und Eingabefelder freizuschalten.

Der [Steuerparameter 850 – Belegänderungssperre durch Beteiligung von openTRANS](../../../../firmenstamm/steuerparameter/optionen_warenwirtschaft/belegaenderungssperre_durch_beteiligung_von_opentrans_spa_85.md) sollte definiert werden, wenn Belege nach Erstellung bzw. Weiterversand an Drittsysteme nicht geändert werden dürfen.

Der [Steuerparameter 508 – Formulararchiv (Lizenz)](../../../../firmenstamm/steuerparameter/lizenzen/archiv_lizenz_spa_508.md) muss eingeschaltet sein, um Dokumente zu verwalten, an die ein openTRANS angehängt werden kann.

Der [Steuerparameter 854 – Nur aktuelle Belege bereitstellen für openTRANS](../../../../firmenstamm/steuerparameter/optionen_global/nur_aktuelle_belege_bereitstellen_fuer_opentrans_spa_854.md) kann eingestellt werden, wenn ins Dateisystem exportierte Dateien transferiert werden sollen.

Der [Steuerparameter 855 – Nur aktuelle Belege bereitstellen für Beleg-Mailversand](../../../../firmenstamm/steuerparameter/optionen_global/nur_aktuelle_belege_bereitstellen_fuer_opentrans_spa_854.md) kann eingestellt werden, wenn mit openTRANS versehene Belege aus dem Archiv gesammelt versendet werden sollen.

Der [Steuerparameter 866 - Preismengeneinheit im openTRANS angeben](../../../../firmenstamm/steuerparameter/optionen_warenwirtschaft/preismengeneinheit_im_opentrans_angeben_spa_866.md) legt fest, ob abweichend vom Standard die Preismengeneinheit aus der Warenposition im XML ausgegeben werden soll. Diese Funktion ist nicht möglich bei Verwendung einer Mengeneinheitsumschlüsselungsprozedur.

<p class="just-emphasize">Mengeneinheiten</p>

Hauptmenü > Stammdaten > Konstanten Artikelstamm > Mengeneinheiten > Variante „internationale Mengeneinheiten“

In der Variante „[internationale Mengeneinheiten](../../../../artikelstamm_und_artikel/konstanten_der_artikelverwaltung/mengeneinheiten_mit_umrechnungen_ergebnismengeneinheit/internationale_mengeneinheit_un.md)“ werden die Zuordnungen von A.eins-eigenen Mengeneinheiten zu internationalen Mengeneinheiten gepflegt. Dabei kann ein Umrechnungsfaktor und eine Voreinstellung angegeben werden.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Internationale Mengeneinheiten</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>UN-Mengeneinheit</p>
        </td>
        <td>
          <p>Auswahl aus Mengeneinheitsangaben der vereinten Nationen gemäß <a href="http://www.unece.org/cefact/recommendations/rec_index.htm">Recommendation N°. 20 - Codes for Units of Measure Used in International Trade</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Mengeneinheit A.eins</p>
        </td>
        <td>
          <p>Mengeneinheit in A.eins</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Faktor UN zu A.eins</p>
        </td>
        <td>
          <p>Umrechnungsfaktor Beispiel: eine Tonne (A.eins zu Kilogramm/KGM international) = 1000</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Voreinstellung</p>
        </td>
        <td>
          <p>Wählen Sie Ja, wenn diese Zuordnung bevorzugt beim Export verwendet werden soll.</p>
          <p>Andere Zuordnungen können beim Import verwendet werden</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

Eine erste Vorbelegung können Sie mit Hilfe der Funktion „Basiseinrichtung“ erstellen. Diese sucht nach gängigen Bezeichnern für Maßeinheiten wie z.B. „m“, „mtr“ oder „meter“ und erstellt daraus eine Zuordnung zur UN-Mengeneinheit – hier im Beispiel MTR mit den entsprechenden Faktoren.

<p class="just-emphasize">Sprache</p>

Im Sprachenpfleger finden Sie die Kundensprachen. Dort muss die ISO-Kennzeichnung der Sprache zugeordnet sein. Anderenfalls wird als Export-Sprache „ger“ für Deutsch angegeben und bei Verwendung der Sprache eine Warnung ins Fehlerprotokoll geschrieben.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Sprachstamm</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Nummer</p>
        </td>
        <td>
          <p>A.eins-interne Nummer für die Sprache</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bezeichnung</p>
        </td>
        <td>
          <p>Bezeichnung für diese Sprache</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>ISO 639-2</p>
        </td>
        <td>
          <p>Iso-Kennzeichnung für diese Sprache nach <a href="http://www.loc.gov/standards/iso639-2/">ISO 639-2</a></p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<p class="just-emphasize">Kunden</p>

Im Kundenstamm muss openTRANS eingerichtet werden.

Siehe dazu Kundenstamm > Kennzeichen > openTRANS

<p class="just-emphasize">Formularzuordnung [FRZ]</p>

In der Formularzuordnung muss für die einzelnen Vorgangsklassen definiert werden, ob openTRANS aktiv sein soll und wohin die Daten geschrieben werden.

Siehe dazu [Vorgangsabwicklung > Formularzuordnung [FRZ] > openTRANS](../../../../vorgangsabwicklung/formularzuordnung/ot_opentrans.md)

**Vorgangsdruckklassen [VRGD]**

Während in der Formularzuordnung genau ein Formular definiert wird, mit dem das Pdf gedruckt wird, das mit dem openTRANS verheiratet wird, so können mittels der Vorgangsdruckklasse mehrere Drucke mit verschiedenen Formularen definiert werden, die beim Druck des Vorgangs verwendet werden. Eines davon muss als jenes ausgezeichnet werden, das mit dem openTRANS-Datensatz verheiratet wird und ggf. später für einen Weiterversand zur Verfügung steht.

Siehe dazu auch [Firmenstamm > Druckereinrichtung > Vorgangsdruckklassen > Formulare / Drucker zuordnen](../../../../firmenstamm/druckereinrichtung/vorgangsdruckklassen.md).

**Formulare**

Die zum Druck mit openTRANS verwendeten Formulare müssen archiviert werden können, da sonst die Zusammenführung von openTRANS-Dokument und gedrucktem Pdf nicht erfolgen kann.

<p class="just-emphasize">Zu- und Abschläge</p>

In allen Zu- und Abschlägen muss ein eigener Zu-Abschlagstyp gepflegt werden. Anderenfalls wird dieser im openTRANS nicht ausgewiesen.

- Bezugsgrößenabhängige Zu-/Abschläge [ZABZ]
- Generelle Zu-/Abschläge [ZAGE]
- Versandartabhängige Zu-/Abschläge [ZAVS]
- Zahlungsartabhängige Zu-/Abschläge [ZAZA]
- Zeitabhängige Zu-/Abschläge [ZAZT]

<p class="just-emphasize">Rabatte</p>

In den Rabattsätzen [RAS] muss ein eigener Zu-Abschlagstyp gepflegt werden. Anderenfalls wird dieser im openTRANS nicht ausgewiesen. In der Regel wird dieser Typ allgemein „Rabatt“ heißen.
