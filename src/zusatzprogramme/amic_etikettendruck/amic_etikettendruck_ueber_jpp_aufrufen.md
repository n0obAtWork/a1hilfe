# AMIC-Etikettendruck über JPP aufrufen

<!-- source: https://amic.de/hilfe/_AMICEtikettendruckJPP.htm -->

Um den AMIC-Etikettendruck programmgesteuert aufzurufen, existiert ein JPP-Objekt mit dem Namen **JEtikettendruck**.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><strong>Methode</strong></p>
        </td>
        <td>
          <p><strong>Parameter</strong></p>
        </td>
        <td>
          <p><strong>Bedeutung</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Version</p>
        </td>
        <td></td>
        <td>
          <p>Liefert die aktuelle Version. Aufruf:</p>
          <div>
            <pre><code>call JPP_NEW("AED","JEtikettenDruck")
  call JPP_DO ("AED", "VERSION", "LDB_TRANSFER$VC")
  call JPP_DEL("AED")</code></pre>
          </div>
          <p>Dabei ist LDB_TRANSFER$VC das Feld auf der Maske, in der die Versionsnummer geschrieben werden soll.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Editieren</p>
        </td>
        <td>
          <p><b>LILAID</b></p>
        </td>
        <td>
          <p>Ruft den interaktiven Designer auf. Beispiel ( JPL-Syntax ):<br><br></p>
          <div>
            <pre><code>call JPP_NEW("AED","JEtikettenDruck")
  call JPP_IN( "AED", "LILAID", "EAN_ETIKETT" )
  call JPP_EX( "AED", "Editieren" )
  call JPP_DEL("AED")</code></pre>
          </div>
        </td>
      </tr>
      <tr>
        <td>
          <p>Vorschau</p>
        </td>
        <td>
          <p><b>LILAID</b></p>
          <p>[procedurecall]</p>
        </td>
        <td>
          <p>Öffnet den Report als Vorschau. Der optionale Parameter überschreibt den Viewnamen bzw. den Prozedurnamen, den man in der Definition angegeben hat.</p>
          <div>
            <pre><code>call JPP_NEW("AED","JEtikettenDruck")
  call JPP_IN( "AED", "LILAID", "EAN_ETIKETT" )
  call JPP_IN( "AED", "procedurecall", "p_kontoblatt(10000,10000,2017,12)" )
  call JPP_EX( "AED", "Vorschau" )
  call JPP_DEL("AED")</code></pre>
          </div>
        </td>
      </tr>
      <tr>
        <td rowspan="6">
          <p>Drucken</p>
        </td>
        <td>
          <p><b>LILAID</b></p>
        </td>
        <td>
          <p>Druck den Report. Beispiel:<br><br></p>
          <div>
            <pre><code>call JPP_NEW("AED","JEtikettenDruck")
  call JPP_IN( "AED", "LILAID", "EAN_ETIKETT" )
  call JPP_IN( "AED", "procedurecall", "p_kontoblatt(10000,10000,2017,12)" )
  call JPP_IN( "AED", "fa_KundNummer", "10000" )
  call JPP_IN( "AED", "fa_BelegDatum", "30.12.2017" )
  call JPP_IN( "AED", "fa_BelegTypText", "Kontoblatt" )
  call JPP_IN( "AED", "ask", "0" )
  call JPP_EX( "AED", "Drucken" )
  call JPP_DEL("AED")</code></pre>
          </div>
        </td>
      </tr>
      <tr>
        <td>
          <p>[ask]</p>
        </td>
        <td>
          <p>0 oder 1, je nachdem, ob vor dem Druck der Drucker bzw. das Ausgabeformat abgefragt werden soll(=1) oder nicht (=0). Standard ist Abfrage(also 1 ).</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>[procedurecall]<b></b></p>
        </td>
        <td>
          <p>Dieser Parameter gibt an, was überhaupt gedruckt werden soll. Das Format muss so sein, wie bei Prozeduren der <a href="./definition_in_a_eins.md#AufrufZumBearbeiten">Aufruf für bearbeiten</a> eingetragen wurde.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>[printerprofil]</p>
        </td>
        <td>
          <p>Man gibt hier einfach das <a href="./definition_in_a_eins.md#Druckerprofile">Profil</a> an, dass man bei der Bearbeitung der Definitionen in A.eins erstellt hat.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>[printername]<b></b></p>
        </td>
        <td>
          <p>Hier gibt man den Drucker direkt mit dem Namen an.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>[fa_KundNummer]</p>
          <p>[fa_BelegNummer]</p>
          <p>[fa_BelegDatum]</p>
          <p>[fa_BelegReferenz]</p>
          <p>[fa_BelegTypText]</p>
          <p>[fa_BelegKlasse]</p>
          <p>[fa_Info_Betreff]</p>
          <p>[fa_Info_Kategorie]</p>
          <p>[fa_Info_Stichwoerter]</p>
          <p>[fa_Info_Autor]</p>
          <p>[fa_Info_Titel]</p>
          <p>[fa_Info_Kommentar]</p>
        </td>
        <td>
          <p>Wird einer dieser optionalen Parameter verwendet, dann werden die Archivierungseinstellungen aus der Definition ignoriert und stattdessen diese Werte verwendet.</p>
        </td>
      </tr>
      <tr>
        <td rowspan="4">
          <p>Exportieren</p>
        </td>
        <td>
          <p><b>LILAID</b></p>
        </td>
        <td>
          <p>Exportiert den Report direkt im angegebenen Format. Ausgabeverzeichnis ist „..\export\lila“<br>Beispiel:</p>
          <div>
            <pre><code>call JPP_NEW("AED","JEtikettenDruck")
  call JPP_IN( "AED", "LILAID", "EAN_ETIKETT" )
  call JPP_IN( "AED", "procedurecall", "p_kontoblatt(10000,10000,2017,12)" )
  call JPP_IN( "AED", "Format", "PDF" )
  call JPP_EX( "AED", "Exportieren" )
  call JPP_DEL("AED")</code></pre>
          </div>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Format</b></p>
        </td>
        <td>
          <p>Format kann folgernde Werte haben: „HTML“, „PDF“, „RTF“, „BMP“, „JPG“</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>[procedurecall]</p>
        </td>
        <td>
          <p>Dieser Parameter gibt an, was überhaupt gedruckt werden soll. Das Format muss so sein, wie bei Prozeduren der <a href="./definition_in_a_eins.md#AufrufZumBearbeiten">Aufruf für bearbeiten</a> eingetragen wurde.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>[defaultApp]</p>
        </td>
        <td>
          <p>Öffnet das Ergebnis mit dem Programm welches mit diesem Format verknüpft ist.</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td></td>
        <td></td>
      </tr>
      <tr>
        <td rowspan="3">
          <p>Archivieren</p>
        </td>
        <td>
          <p><b>LILAID</b></p>
        </td>
        <td>
          <p>Archiviert den Report ohne vorher zu drucken. Beispiel:<br><br></p>
          <div>
            <pre><code>call JPP_NEW("AED","JEtikettenDruck")
  call JPP_IN( "AED", "LILAID", "EAN_ETIKETT")
  call JPP_IN( "AED", "procedurecall", "p_kontoblatt(10000,10000,2017,12)" )
  call JPP_IN( "AED", "fa_KundNummer", "10000" )
  call JPP_IN( "AED", "fa_BelegDatum", "30.12.2017" )
  call JPP_IN( "AED", "fa_BelegTypText", "Kontoblatt" )
  call JPP_EX( "AED", "Archivieren" )
  call JPP_DEL("AED")</code></pre>
          </div>
        </td>
      </tr>
      <tr>
        <td>
          <p>[procedurecall]</p>
        </td>
        <td>
          <p>Dieser Parameter gibt an, was überhaupt gedruckt werden soll. Das Format muss so sein, wie bei Prozeduren der <a href="./definition_in_a_eins.md#AufrufZumBearbeiten">Aufruf für bearbeiten</a> eingetragen wurde.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>[fa_KundNummer]</p>
          <p>[fa_BelegNummer]</p>
          <p>[fa_BelegDatum]</p>
          <p>[fa_BelegReferenz]</p>
          <p>[fa_BelegTypText]</p>
          <p>[fa_BelegKlasse]</p>
          <p>[fa_Info_Betreff]</p>
          <p>[fa_Info_Kategorie]</p>
          <p>[fa_Info_Stichwoerter]</p>
          <p>[fa_Info_Autor]</p>
          <p>[fa_Info_Titel]</p>
          <p>[fa_Info_Kommentar]</p>
        </td>
        <td>
          <p>Wird einer dieser optionalen Parameter verwendet, dann werden die Archivierungseinstellungen aus der Definition ignoriert und stattdessen diese Werte verwendet.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
