# Anwendung

<!-- source: https://amic.de/hilfe/_ prodI_Anwendung.htm -->

Gelesene Dateien werden zunächst serialisiert in den Typ „AMIC.ProduktionsInterface.Produktion“ und dann an die Methode „VerarbeiteRueckMeldung“ der Klasse „ProduktionsAenderung“ übergeben.

```csharp
XmlSerializer ser;
StreamReader reader = null;
Produktion prod = null;
Encoding enc = Encoding.GetEncoding("ISO-8859-1");
try
{
  reader = new StreamReader(filename);
  ser = new XmlSerializerFactory().CreateSerializer(typeof(Produktion));
  prod  = (Produktion)ser.Deserialize(reader);
  reader.Close();
}
finally
{
  if (reader != null)
  {
    reader.Close();
  }
}
//Nur weiter, wenn eine Produktionsänderung erkannt wurde
if (prod == null)
{
  return;
}
int bedienerMand = D.GetExecuteScalar<int>(-1, "select bedienerid from bedienerstamm where BedienerKurz = 'MAND'");
ProduktionsAenderung pa = new ProduktionsAenderung();
pa.VerarbeiteRueckMeldung(prod, bedienerMand);
```

Zu Exportierende Daten werden zunächst serialisiert und dann in eine Datei geschrieben

```csharp
ProduktionsExport pe = new ProduktionsExport();
Produktion prod = pe.ExportData(v_id, ProdStatus.Rezept);
XmlSerializer ser;
XmlTextWriter xmlTextWriter = null;
string XML = "";
MemoryStream ms = null;
Encoding enc = Encoding.GetEncoding("ISO-8859-1");
try
{
  ms = new MemoryStream();
  xmlTextWriter = new XmlTextWriter(ms, enc);
  ser = new XmlSerializerFactory().CreateSerializer(typeof(Produktion));
  ser.Serialize(xmlTextWriter, prod);
  //Memorystream füllen
  ms = (MemoryStream)xmlTextWriter.BaseStream;
  //XML als String ermitteln
  ms.Seek(0, SeekOrigin.Begin);
  XML = enc.GetString(ms.ToArray());
}
finally
{
  if (ms != null)
  {
    ms.Close();
  }
}
///Es muss ein XML geschrieben worden sein
if (string.IsNullOrEmpty(XML))
{
  return;
}
//String wegschreiben.
StreamWriter sw = null;
try
{
  sw = new StreamWriter(Path.Combine(exportPfad, "parts_"+v_id.ToString()+".xml"), false, Encoding.GetEncoding("ISO-8859-1"));
  sw.Write(XML);
  sw.Flush();
}
finally
{
  if (sw != null)
  {
    sw.Flush();
    sw.Close();
  }
}
```
