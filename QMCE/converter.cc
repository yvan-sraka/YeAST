// ---- Converter Modules ---- //

/*
int MainWindow::convert(QString path, QString language)
{
    if (language == "Python")
    {
        QFile fichier0(path + "/html/index.php");

        if (fichier0.open(QIODevice::WriteOnly | QIODevice::Text))
        {
            fichier0.write("<!DOCTYPE html>\n" \
                           "<html lang=\"en\">\n" \
                           "<head>\n" \
                           "\t <meta charset=\"utf-8\" />\n" \
                           "</head>\n" \
                           "<body>\n" \
                           "<script>\n" \
                           "\t var code = \"print \\\"Hello World !\\\"\";\n" \
                           "\t /print ['\\\"](\\S.*)['\\\"]/.exec(code);\n" \
                           "\t code = \"<p>\" + RegExp.$1 + \"</p>\";\n" \
                           "</script>\n" \
                           "</body>\n" \
                           "</html>\n");

            fichier0.close();
        }
    }

    else if (language == "JS")
    {
        QFile fichier0(path + "/html/index.php");

        if (fichier0.open(QIODevice::WriteOnly | QIODevice::Text) )
        {
            fichier0.write("<!DOCTYPE html>\n" \
                           "<html lang=\"en\">\n" \
                           "<head>\n" \
                           "\t <meta charset=\"utf-8\" />\n" \
                           "</head>\n" \
                           "<body>\n" \
                           "<script src= \"main.js\"></script>\n" \
                           "</body>\n" \
                           "</html>\n");
            fichier0.close();
        }
    }

    else
        QMessageBox::warning(this, tr("Warning"), tr("No module corresponds to the requested action ..."));

    return 0;
}
*/
