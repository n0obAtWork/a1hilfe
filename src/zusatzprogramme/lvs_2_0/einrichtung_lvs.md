# Einrichtung LVS

<!-- source: https://amic.de/hilfe/_lvs20_einr.htm -->

#### Ladeträgertypen

Hauptmenü > Stammdatenpflege > Lagerverwaltungssystem > Ladeträgertypen

oder Direktsprung [[LVLTT](../../firmenstamm/lagerverwaltungssystem/ladetraegertyp.md)]

Es müssen Ladeträgertypen eingerichtet werden. Es empfiehlt sich folgende Einrichtung vorzunehmen bzw. zu ergänzen:

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="3">
          <p><strong>Ladeträgertypen</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p><strong>Typ</strong></p>
        </td>
        <td>
          <p><strong>Typ</strong></p>
        </td>
        <td>
          <p><strong>Breite</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>2</p>
        </td>
        <td>
          <p>EUR1</p>
        </td>
        <td>
          <p>2 – 80x120 (für EURO-Paletten)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>EUR3</p>
        </td>
        <td>
          <p>3 – 120x120 (für BigBags)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>99</p>
        </td>
        <td>
          <p>Linie</p>
          <p>(siehe auch <a href="./einrichtung_lvs.md#LVS_Einrichtung_SPA">Einrichtung SPA 1037</a>)</p>
        </td>
        <td>
          <p>undefiniert</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

#### Lokalitäten

Hauptmenü > Stammdatenpflege > Lagerverwaltungssystem > Regale (Lokalitäten)

oder Direktsprung [[LVSLK](../../firmenstamm/lagerverwaltungssystem/lokalitaeten/index.md)]

Die Lokalitäten sind die Stellplätze in einem Lager. Sie können manuell mit [[LVSLK](../../firmenstamm/lagerverwaltungssystem/lokalitaeten/index.md)] eingerichtet werden. Lokalitäten im Regal können zu einer Lokalitäten-Gruppe [[LVLKG](../../firmenstamm/lagerverwaltungssystem/lokalitaetengruppe.md)] zusammengefasst werden.

Es empfiehlt sich, die Lokalitäten mit einem Skript einzurichten.

Die Lokalitätsnummer 999.999 ist als Leerpaletten-Lokalität anzulegen!

**Lokalitätstyp**

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="3">
          <p><b>Lokalitätstyp</b></p>
          <p>Bei der Anlage der Lokalitäten muss ein Typ angegeben werden.</p>
          <p><strong>Die folgende Enumeration ist dabei zu verwenden:</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Nr</b></p>
        </td>
        <td>
          <p><b>Bezeichnung</b></p>
        </td>
        <td>
          <p><b>Bemerkung</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>10</p>
        </td>
        <td>
          <p>Wareneingang</p>
        </td>
        <td>
          <p>Ankommende Waren</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>20</p>
        </td>
        <td>
          <p>Warenausgang</p>
        </td>
        <td>
          <p>Ausgehende Waren</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>30</p>
        </td>
        <td>
          <p>Kommissionierbereich</p>
        </td>
        <td>
          <p>Angebrochene Paletten – Lokalität</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>40</p>
        </td>
        <td>
          <p>Produktionslager</p>
        </td>
        <td>
          <p>Material, das an der Linie lagert</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>42</p>
        </td>
        <td>
          <p>Linie Bereitstellung</p>
        </td>
        <td>
          <p>Bereitstellungsbereich für die Produktion</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>44</p>
        </td>
        <td>
          <p>Linie Fertigware</p>
        </td>
        <td>
          <p>Bereich der fertiggestellten Waren</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>46</p>
        </td>
        <td>
          <p>Linie</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>50</p>
        </td>
        <td>
          <p>Regalplatz</p>
        </td>
        <td>
          <p>Regalplatz in einem (Hochregal)-Lager</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>52</p>
        </td>
        <td>
          <p>Blocklager-Bereich</p>
        </td>
        <td>
          <p>Lagerfläche mit wahlfreiem Zugriff</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>54</p>
        </td>
        <td>
          <p>Gefahrstofflager</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>70</p>
        </td>
        <td>
          <p>Pufferbereich</p>
        </td>
        <td>
          <p>Zwischenlager / Abstellbereich</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>80</p>
        </td>
        <td>
          <p>Sperrlager</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>90</p>
        </td>
        <td>
          <p>Außenlager</p>
        </td>
        <td>
          <p>Lager ohne LVS-Kontrolle</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>96</p>
        </td>
        <td>
          <p>Scanner</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>97</p>
        </td>
        <td>
          <p>Schwundlager</p>
        </td>
        <td>
          <p>Buchungsplatz für nicht auffindbare Waren</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>98</p>
        </td>
        <td>
          <p>LKW/Trecker/In Transit</p>
        </td>
        <td>
          <p>Waren, die derzeit transportiert werden</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>99</p>
        </td>
        <td>
          <p>Scanner</p>
        </td>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>

Mit Ausnahme des Regal-Lagerplatzes sind alle Lokalitäten groß genug unendlich viele Ladeträger aufzunehmen. Die Größe wird nicht begrenzt. Nur ein Regal-Lagerplatz ist ausschließlich mit einem Ladeträger zu belegen.

**Produktionslinien**

Hauptmenü > Stammdatenpflege > Lagerverwaltungssystem > Produktionslinien

oder Direktsprung [[PRODL]](../../firmenstamm/produktionslinien.md)

Die o.g. Produktionslinien lassen sich im LVS an Produktionsleitsysteme anbinden. Dazu ist eine Kommunikation im Dateiaustausch mittels XML-Dateien vorgesehen. Das Drittsystem kann Material anfordern oder fertig melden. Die Lokalitäten werden mit dem Interface-Namen der Linie in den Produktionslinienpfleger eingetragen [[PRODL]](../../firmenstamm/produktionslinien.md)

**Bediener für Scanner**

Hauptmenü > Administration > Firmenkonstanten > Bediener

oder Direktsprung [BD]

Für die Scanner müssen Bediener eingerichtet werden. In der Regel wird man für jeden Scanner einen Bediener anlegen.

**Steuerparameter**

Hauptmenü > Administration > Steuerung > Steuerparameter zeigen

oder Direktsprung **[SPA]**

Es gibt eine Reihe von Steuerparametern für LVS 2.0:

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="3">
          <p><b>Steuerparameter</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Nr</b></p>
        </td>
        <td>
          <p><b>Bezeichnung</b></p>
        </td>
        <td>
          <p><b>Werte</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p><a href="../../firmenstamm/steuerparameter/optionen_global/lagerverwaltungssystem_spa_636.md">636</a></p>
        </td>
        <td>
          <p>LVS Aktiv</p>
        </td>
        <td>
          <p>auf „Ja“</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><a href="../../firmenstamm/steuerparameter/scanner/private_scannerprozedur_spa_801.md">801</a></p>
        </td>
        <td>
          <p>Private Scannerprozedur</p>
        </td>
        <td>
          <p>Für JEDEN Scanner und seine IP-Adresse muss hier „AMIC_LVS_WORKFLOW“ eingerichtet werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><a href="../../firmenstamm/steuerparameter/lizenzen/permanente_inventur_lizenz_spa_902.md">902</a></p>
        </td>
        <td>
          <p>Lizenz Permanente Inventur</p>
        </td>
        <td>
          <p>n/a</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><a href="../../firmenstamm/steuerparameter/allgemeine_programmsteuerung/cs_makro_debugsession_fuer_bediener_erhalten_spa_941.md">941</a></p>
        </td>
        <td>
          <p>CSMakro DebugSession</p>
        </td>
        <td>
          <p>Für Bediener AMIC auf „Ja“</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><a href="../../firmenstamm/steuerparameter/optionen_global/lvs_ladetraeger_umbuchung_aktiv_spa_946.md">946</a></p>
        </td>
        <td>
          <p>LVS Umbuchung aktiv</p>
        </td>
        <td>
          <p>Auf „Nein“</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><a href="../../firmenstamm/steuerparameter/allgemeine_programmsteuerung/lvs_fahrauftrag_verwenden_spa_947.md">947</a></p>
        </td>
        <td>
          <p>LVS Fahraufträge verwenden</p>
        </td>
        <td>
          <p>Auf „Ja“</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><a href="../../firmenstamm/steuerparameter/scanner/html_seite_als_vollbild_spa_953.md">953</a></p>
        </td>
        <td>
          <p>HTML-Seite als Vollbild darstellen</p>
        </td>
        <td>
          <p>Für JEDEN Scanner und seine IP-Adresse muss hier „Ja“ eingestellt werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><a href="../../firmenstamm/steuerparameter/vorgangsbearbeitung_umwandlung/teildisposition_mit_vorlauf_aktiv_spa_986.md">986</a></p>
        </td>
        <td>
          <p>Teildisposition mit Vorlauf aktiv</p>
        </td>
        <td>
          <p>Auf „Nein“</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><a href="../../firmenstamm/steuerparameter/allgemeine_programmsteuerung/lvs_workflow_prozeduren_spa_1029.md">1029</a></p>
        </td>
        <td>
          <p>LVS Workflow-Prozeduren</p>
          <p>LVS-Kommandos</p>
        </td>
        <td>
          <p>Hier kann für „LVS-Kommandos“ die private Scantyp-Prozedur eingetragen werden</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>1037</p>
        </td>
        <td>
          <p>Ladeträgertyp Produktionssilo</p>
        </td>
        <td>
          <p><a href="./einrichtung_lvs.md#LVS_Einrichtung_LTT">Gemäß Einrichtung Ladeträgertyp Linie</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p><a href="../../firmenstamm/steuerparameter/vorgangsbearbeitung_allg/lvs_allokationsstrategie_spa_1038.md">1038</a></p>
        </td>
        <td>
          <p>LVS Allokationsstrategie</p>
        </td>
        <td>
          <p>1.&nbsp;&nbsp; Kommi zuerst</p>
          <p>2.&nbsp;&nbsp; Vollpaletten zuerst</p>
          <p>3.&nbsp;&nbsp; Alles zugleich</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><a href="../../firmenstamm/steuerparameter/optionen_global/gln_spa_1042.md">1042</a></p>
        </td>
        <td>
          <p>GLN</p>
        </td>
        <td>
          <table>
            <tbody>
              <tr>
                <th>GLN</th>
                <th>GLN des Unternehmens oder 000000000</th>
              </tr>
              <tr>
                <td>Basisnummer</td>
                <td>Die ersten 7 Ziffern der GLN oder 0000000</td>
              </tr>
              <tr>
                <td>NVE Reserveziffer</td>
                <td>Vorschlag: 1</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p><a href="../../firmenstamm/steuerparameter/optionen_warenwirtschaft/permanente_inventur_mit_lvs_spa_1045.md">1045</a></p>
        </td>
        <td>
          <p>Permanente Inventur</p>
        </td>
        <td>
          <ul>
            <li>Zählzeitraum in Tagen</li>
            <li>Anzahl Zeilen pro Inventurbeleg</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p><a href="../../firmenstamm/steuerparameter/mde_prozeduren_einzelhandel_spa_1059/lvs_meldungen_unterdruecken_spa_1060.md">1060</a></p>
        </td>
        <td>
          <p>LVS Meldungen unterdrücken</p>
        </td>
        <td>
          <p>Muss bei LVS 2.0 auf 1 stehen</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

**Vorgangsunterklassen LVS**

Hauptmenü > Administration > Formulare/Abläufe > Formularzuordnung / Vorgangsunterklasse

oder Direktsprung **[FRZ]**

Das Lagerverwaltungssystem ist kein Warenvorgang wie z.B. ein Lieferschein. Dennoch wird es im Vorgangsimport wegen der ähnlichen Datenstruktur so behandelt. Aus diesem Grund müssen Vorgangsunterklassen mit der Datenbankfunktion „AMIC_LVS_INSTALL_FRZ“ einmalig angelegt und ggf. angepasst werden.

**MDE-Workflow**

Hauptmenü > Stammdatenpflege > Lagerverwaltungssystem > MDE-Workflow

oder Direktsprung **[LVSWF]**

In **[LVSWF]** ist das [LVS-Workflow](../../firmenstamm/lvs_mde_workflow.md) einzurichten.

Es gibt ein Standard-Workflow, das jedoch stets zu individualisieren ist.

Dort gilt es auch private Scancodes in einer privaten Datenbankfunktion zu hinterlegen.
