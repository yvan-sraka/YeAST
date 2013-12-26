#include <iostream>

//extern std::istream std::cin;

int main(int argc, char *argv[])
{
    /* -- Recovery cmds entered -- */

    std::string c, lang, path;

    if (argc == 1)
    {
        std::cout << "Enter a cmd : ";
		//std::cin >> c;
    }

    /* -- Help cmd -- */

    if (c == "help" || std::string(argv[0]) == "--help")
    {
		std::cout << std::endl << "*** QMCE Help : ***" << std::endl;
        std::cout << "build : displays help." << std::endl;
        std::cout << "help : build function." << std::endl << std::endl;
    }

    /* -- Build cmd -- */

    if (c == "build" || std::string(argv[0]) == "-build")
    {
        std::cout << "Choose a language : ";
        //std::cin >> lang;

        std::cout << "Choose a path : ";
        //std::cin >> path;

        std::string cmd = "";

        if (lang == "C")
        {
#ifdef Q_WS_X11
            cmd = "cd \"" + path + "\" && gcc *.c";
#endif
        }

        else if (lang == "C++")
        {
#ifdef Q_WS_X11
            cmd = "cd \"" + path + "\" && g++ *.cpp";
#endif
        }

        else if (lang == "Qt")
        {
#ifdef Q_WS_X11
            // WIN : cd '$FOLDER$' && C:\$QTDIR$\qt\bin\qmake.exe && C:\$QTDIR$\mingw\bin\mingw32-make.exe && pause
            // X11 : cd '$FOLDER$' && qmake && make
            cmd = "cd \""+path+"\" && qmake && make";
#endif
        }

        else if (lang == "Java")
        {
#ifdef Q_WS_X11
            cmd = "cd \"" + path + "\" && javac *.java";
#endif
        }

        else if (lang == "Android")
        {
#ifdef Q_WS_X11
            cmd = "cd \""+path+"\" && ant build";
#endif
        }

        else if (lang == "AS3")
        {
#ifdef Q_WS_X11
            cmd = "cd \"" + path + "\" && mxmlc *.as";
            // WIN : cd '$FOLDER$\flex_sdk\bin\' && bash mxmlc '$FOLDER$\$FILENAME$.as' -default-size '$WIDHT$' '$HEIGHT$'
            // X11 : cd '$FOLDER$/flex_sdk/bin' && sh mxmlc '$FOLDER$/$FILENAME$.as' -default-size '$WIDHT$' '$HEIGHT$'
#endif
        }

        else
            std::cout << "WARNING : No module corresponds to the requested action ..." << std::endl;

        system(cmd.c_str());
    }

    return 0;
}

/* // ---- XMotor Function ---- //

void MainWindow::xmotor(QString title, QString icon, QString url, int  width, int height, QString mode)
{
    QWidget *xmotor = new QWidget();
    QWebView *xmotorWebView = new QWebView(xmotor);

    QWebSettings::globalSettings()->setAttribute(QWebSettings::PluginsEnabled, true);
    QWebSettings::globalSettings()->setAttribute(QWebSettings::JavascriptEnabled, true);

    xmotor->setWindowTitle(title);
    xmotor->setWindowIcon(QIcon(QDir::currentPath() + icon));

    if (mode == "tool")
        xmotor->setWindowFlags(Qt::Tool);

    if (url == "localhost")
        xmotorWebView->load(QUrl(QDir::currentPath()));
    else
        xmotorWebView->load(QUrl(url));

    xmotor->setFixedSize(width, height);
    xmotorWebView->setGeometry(0, 0, width, height);

    xmotor->show();
}


// ---- Builder Modules ---- //

void MainWindow::buildSlot()
{
    if (projectLanguage == "HTML")
    {
        generate(projectPath, "Desktop", projectName, "", projectUrl, projectWidth, projectHeight, projectMode);
        build(projectPath, "Qt");
        generate(projectPath, "Android", projectName, "", projectUrl, projectWidth, projectHeight, projectMode);
        build(projectPath, "Android");
    }

    else if (projectLanguage == "PHP")
    {
        generate(projectPath, "Desktop", projectName, "", projectUrl, projectWidth, projectHeight, projectMode);
        build(projectPath, "Qt");
        generate(projectPath, "Android", projectName, "", projectUrl, projectWidth, projectHeight, projectMode);
        build(projectPath, "Android");
    }

    else if (projectLanguage == "JS")
    {
        convert(projectPath, "JS");
        generate(projectPath, "Desktop", projectName, "", projectUrl, projectWidth, projectHeight, projectMode);
        build(projectPath, "Qt");
        generate(projectPath, "Android", projectName, "", projectUrl, projectWidth, projectHeight, projectMode);
        build(projectPath, "Android");
    }

    else if (projectLanguage == "C")
        build(projectPath, "C");

    else if (projectLanguage == "C++")
        build(projectPath, "C++");

    else if (projectLanguage == "Qt")
        build(projectPath, "Qt");

    else if (projectLanguage == "Python")
    {
        convert(projectPath, "Python");
        generate(projectPath, "Desktop", projectName, "", projectUrl, projectWidth, projectHeight, projectMode);
        build(projectPath, "Qt");
        generate(projectPath, "Android", projectName, "", projectUrl, projectWidth, projectHeight, projectMode);
        build(projectPath, "Android");
    }

    else if (projectLanguage == "Java")
    {
        build(projectPath, "Java");
    }

    else if (projectLanguage == "Android")
    {
        build(projectPath, "Android");
    }

    else if (projectLanguage == "AS3")
    {
        build(projectPath, "AS3");
        generate(projectPath, "Desktop", projectName, "", projectUrl, projectWidth, projectHeight, projectMode);
        build(projectPath, "Qt");
        generate(projectPath, "Android", projectName, "", projectUrl, projectWidth, projectHeight, projectMode);
        build(projectPath, "Android");
    }

    else
        QMessageBox::warning(this, tr("Warning"), tr("No module corresponds to the requested action ..."));
}

void MainWindow::debugSlot()
{
    debug(projectPath, projectLanguage);
}

int MainWindow::debug(QString path, QString language)
{
    QString command = "";
}


// ---- Generator Modules ---- //

void MainWindow::generate(QString path, QString system, QString title, QString icon, QString url, QString  width, QString height, QString mode)
{
    if (system == "Desktop")
    {
        QFile fichier0(path + "/" + title + ".pro");

        if (fichier0.open(QIODevice::WriteOnly | QIODevice::Text))
        {
            fichier0.write(("QT += core gui network webkit xml\n\n" \
                            "TARGET = " + title + "\n" \
                            "TEMPLATE = app\n\n\n" \
                            "SOURCES += main.cpp mainwindow.cpp\n\n" \
                            "HEADERS  += mainwindow.h\n").toUtf8());

            fichier0.close();
        }

        QFile fichier1(path + "/main.cpp");

        if (fichier1.open(QIODevice::WriteOnly | QIODevice::Text))
        {
            fichier1.write("#include <QtGui/QApplication>\n" \
                           "#include \"mainwindow.h\"\n\n" \
                           "int main(int argc, char *argv[])\n" \
                           "{\n" \
                           "\t QApplication a(argc, argv);\n" \
                           "\t MainWindow w;\n" \
                           "\t w.show();\n\n" \
                           "\t return a.exec();\n" \
                           "}\n");

            fichier1.close();
        }

        QFile fichier2(path + "/mainwindow.cpp");

        if (fichier2.open(QIODevice::WriteOnly | QIODevice::Text))
        {
            fichier2.write(("#include \"mainwindow.h\"\n\n"  \
                            "#include <QtWebKit/QWebView>\n" \
                            "#include <QDir>\n\n" \
                            "MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent)\n" \
                            "{\n" \
                            "\t title = \"" + title + "\";\n" \
                            "\t url = \"" + url + "\";\n" \
                            "\t width = " + width + ";\n" \
                            "\t height = " + height + ";\n" \
                            "\t mode = \"" + mode + "\";\n\n\n" \
                            "\t QWidget *xmotor = new QWidget();\n" \
                            "\t QWebView *xmotorWebView = new QWebView(xmotor);\n\n" \
                            "\t QWebSettings::globalSettings()->setAttribute(QWebSettings::PluginsEnabled, true);\n" \
                            "\t QWebSettings::globalSettings()->setAttribute(QWebSettings::JavascriptEnabled, true);\n\n" \
                            "\t xmotor->setWindowTitle(title);\n" \
                            "\t xmotor->setWindowIcon(QIcon(QDir::currentPath() + icon));\n\n" \
                            "\t if (mode == \"tool\") xmotor->setWindowFlags(Qt::Tool);\n\n" \
                            "\t if (url == \"localhost\") xmotorWebView->load(QUrl(QDir::currentPath() + \"/index.html\"));\n" \
                            "\t else xmotorWebView->load(QUrl(url));\n\n" \
                            "\t xmotor->setFixedSize(width, height);\n" \
                            "\t xmotorWebView->setGeometry(0, 0, width, height);\n\n" \
                            "\t setCentralWidget(xmotor);\n" \
                            "}\n\n" \
                            "MainWindow::~MainWindow()\n" \
                            "{\n\n" \
                            "}\n").toUtf8());

            fichier2.close();
        }

        QFile fichier3(path + "/mainwindow.h");

        if (fichier3.open(QIODevice::WriteOnly | QIODevice::Text))
        {
            fichier3.write("#ifndef MAINWINDOW_H\n" \
                           "#define MAINWINDOW_H\n\n" \
                           "#include <QtGui/QMainWindow>\n\n" \
                           "class MainWindow : public QMainWindow\n" \
                           "{\n" \
                           "\t Q_OBJECT\n\n" \
                           "public:\n" \
                           "\t MainWindow(QWidget *parent = 0);\n" \
                           "\t ~MainWindow();\n\n" \
                           "private:\n" \
                           "\t QString title;\n" \
                           "\t QString icon;\n" \
                           "\t QString url;\n" \
                           "\t int width;\n" \
                           "\t int height;\n" \
                           "\t QString mode;\n" \
                           "};\n\n" \
                           "#endif // MAINWINDOW_H\n");

            fichier3.close();
        }
    }

    else if (system == "Android")
    {
        QFile fichier0(path + "/build.xml");

        if (fichier0.open(QIODevice::WriteOnly | QIODevice::Text))
        {
            fichier0.write(("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n" \
                            "<manifest xmlns:android=\"http://schemas.android.com/apk/res/android\"\n" \
                            "\t package=\"com.mobile." + projectName + "\"\n" \
                            "\t android:versionCode=\"1\"\n" \
                            "\t android:versionName=\"1.0\">\n" \
                            "\t <uses-sdk android:minSdkVersion=\"4\" />\n" \
                            "\t <uses-permission android:name=\"android.permission.INTERNET\" />\n" \
                            "\t <application android:icon=\"@drawable/icon\" android:label=\"@string/app_name\">\n" \
                            "\t <activity android:name=\".xmotorActivity\"\n" \
                            "\t\t android:label=\"@string/app_name\">\n" \
                            "\t\t <intent-filter>\n" \
                            "\t\t\t <action android:name=\"android.intent.action.MAIN\" />\n" \
                            "\t\t\t <category android:name=\"android.intent.category.LAUNCHER\" />\n" \
                            "\t\t </intent-filter>\n" \
                            "\t </activity>\n\n" \
                            "\t </application>\n" \
                            "</manifest>\n").toUtf8());

            fichier0.close();
        }


        QFile fichier1(path + "/default.properties");

        if (fichier1.open(QIODevice::WriteOnly | QIODevice::Text))
        {
            fichier1.write("# This file is automatically generated by Android Tools.\n" \
                           "# Do not modify this file -- YOUR CHANGES WILL BE ERASED!\n" \
                           "#\n" \
                           "# This file must be checked in Version Control Systems.\n" \
                           "#\n" \
                           "# To customize properties used by the Ant build system use,\n" \
                           "# \"build.properties\", and override values to adapt the script to your\n" \
                           "# project structure.\n\n" \
                           "# Project target.\n" \
                           "target=android-4\n");

            fichier1.close();
        }


        QFile fichier2(path + "/src/com/mobile/" + projectName + "/" + projectName + "Activity.java");

        if (fichier2.open(QIODevice::WriteOnly | QIODevice::Text))
        {
            fichier2.write(("package com.mobile." + projectName + ";\n\n" \
                            "import android.app.Activity;\n" \
                            "import android.os.Bundle;\n" \
                            "import android.util.Log;\n" \
                            "import android.view.KeyEvent;\n" \
                            "import android.view.View;\n" \
                            "import android.webkit.WebView;\n" \
                            "import android.webkit.WebViewClient;\n\n" \
                            "public class xmotorActivity extends Activity {\n" \
                            "\t Called when the activity is first created. \n" \
                            "\t @Override\n" \
                            "\t public void onCreate(Bundle savedInstanceState) {\n" \
                            "\t\t super.onCreate(savedInstanceState);\n\n" \
                            "\t\t WebView store = new WebView(this);\n" \
                            "\t\t store.getSettings().setJavaScriptEnabled(true);\n" \
                            "\t\t store.setWebViewClient(new xmotorWebClient());\n\n" \
                            "\t\t store.loadUrl(\"" + projectUrl +"\");\n" \
                            "\t\t setContentView(store);//setContentView(R.layout.main);\n\n" \
                            "\t }\n\n" \
                            "\t public boolean onKeyDown(int keyCode, KeyEvent event) {\n\n" \
                            "\t\t Log.e(\"KEY PRESSED: \",getString(keyCode));\n" \
                            "\t\t return true;\n" \
                            "\t }\n\n" \
                            "\t private class xmotorWebClient extends WebViewClient {\n" \
                            "\t\t @Override\n" \
                            "\t\t public boolean shouldOverrideUrlLoading(WebView view, String url) {\n" \
                            "\t\t\t view.loadUrl(url);\n" \
                            "\t\t\t return true;\n" \
                            "\t\t }\n" \
                            "\t }\n" \
                            "}\n").toUtf8());

            fichier2.close();
        }


        QFile fichier3(path + "/res/layout/main.xml");

        if (fichier3.open(QIODevice::WriteOnly | QIODevice::Text))
        {
            fichier3.write("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n" \
                           "<LinearLayout xmlns:android=\"http://schemas.android.com/apk/res/android\"\n" \
                           "\t android:orientation=\"vertical\"\n" \
                           "\t android:layout_width=\"fill_parent\"\n" \
                           "\t android:layout_height=\"fill_parent\"\n" \
                           "\t >\n" \
                           "<TextView\n" \
                           "\t android:layout_width=\"fill_parent\"\n"  \
                           "\t android:layout_height=\"wrap_content\"\n" \
                           "\t android:text=\"@string/hello\"\n" \
                           "\t />\n" \
                           "</LinearLayout>\n");

            fichier3.close();
        }


        QFile fichier4(path + "/res/values/strings.xml");

        if (fichier4.open(QIODevice::WriteOnly | QIODevice::Text))
        {
            fichier4.write(("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n" \
                            "<resources>\n" \
                            "\t <string name=\"hello\">Hello World, Activity!</string>\n" \
                            "\t <string name=\"app_name\">" + projectName + "</string>\n" \
                            "</resources>\n").toUtf8());

            fichier4.close();
        }
    }

    else if (system == "WebOS")
    {
        QFile fichier0(path + "/appinfo.json");

        if (fichier0.open(QIODevice::WriteOnly | QIODevice::Text))
        {
            fichier0.write(("{" \
                            "\t \"id\": \"com.mobile.xmotor\",\n" \
                            "\t \"version\": \"1.0.0\",\n" \
                            "\t \"vendor\": \"\",\n" \
                            "\t \"type\": \"web\",\n" \
                            "\t \"main\": \"index.html\",\n" \
                            "\t \"title\": \"" + projectName + "\",\n" \
                            "\t \"icon\": \"" + projectIcon + "\",\n" \
                            "\t \"uiRevision\": \"2\"\n" \
                            "}\n").toUtf8());

            fichier0.close();
        }


        QFile fichier1(path + "/framework_config.json");

        if (fichier1.open(QIODevice::WriteOnly | QIODevice::Text))
        {
            fichier1.write("{" \
                           "\t \"logLevel\": 99,\n" \
                           "\t \"debuggingEnabled\": true,\n" \
                           "\t \"timingEnabled\": false,\n" \
                           "\t \"logEvents\": false,\n" \
                           "\t \"escapeHTMLInTemplates\" : true\n" \
                           "}\n");

            fichier1.close();
        }


        QFile fichier2(path + "/index.html");

        if (fichier2.open(QIODevice::WriteOnly | QIODevice::Text))
        {
            fichier2.write("<!DOCTYPE html>/n"
                           "<html>/n"
                           "<head>/n"
                           "/t <title>xmotor</title>/n"
                           "/t <script src=\"/usr/palm/frameworks/mojo/mojo.js\" type=\"text/javascript\" x-mojo-version=\"1\"></script>/n/n"
                           "/t <!-- application stylesheet should come in after the one loaded by the framework -->/n"
                           "/t <link href=\"stylesheets/xmotor.css\" media=\"screen\" rel=\"stylesheet\" type=\"text/css\">/n"
                           "</head>/n"
                           "</html>/n");

            fichier2.close();
        }


        QFile fichier3(path + "/res/layout/main.xml");

        if (fichier3.open(QIODevice::WriteOnly | QIODevice::Text))
        {
            fichier3.write("[\n"
                           "\t {\"source\": \"app/assistants/stage-assistant.js\"},\n"
                           "\t {\n"
                           "\t\t \"scenes\": \"WebView\",\n"
                           "\t\t \"source\": \"app/assistants/WebView-assistant.js\"\n"
                           "\t },\n"
                           "\t {\n"
                           "\t\t \"scenes\": \"errorWifi\",\n"
                           "\t\t \"source\": \"app/assistants/errorWifi-assistant.js\"\n"
                           "\t }\n"
                           "]\n");

            fichier3.close();
        }
    }

    else
        QMessageBox::warning(this, tr("Warning"), tr("No module corresponds to the requested action ..."));
}


// ---- Converter Modules ---- //

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

// ---- Simulator Modules ---- //

// Linux :

/* <simulate>
   <title value="Linux" />
   <url value="http://bellard.org/jslinux/" />
   <type value="web" />
   <width value="800" />
   <height value="500" />
   </simulate> */


// NES :

/* <simulate>
   <title value="NES" />
   <url value="http://benfirshman.com/projects/jsnes/" />
   <type value="web" />
   <width value="800" />
   <height value="500" />
   </simulate> */

/*
   void MainWindow::slotA1()
   {
       xmotor("Samsung Nexus S", "", lineEditUrl->text(), 480, 800, "");
   }

   void MainWindow::slotA2()
   {
       xmotor("Samsung Galaxy S", "", lineEditUrl->text(), 480, 800, "");
   }

   void MainWindow::slotA3()
   {
       xmotor("Apple IPhone 4", "", lineEditUrl->text(), 640, 960, "");
   }

   void MainWindow::slotA4()
   {
       xmotor("Apple Iphone 3GS", "", lineEditUrl->text(), 320, 480, "");
   }

   void MainWindow::slotA5()
   {
       xmotor("Motorola Atrix", "", lineEditUrl->text(), 540, 960, "");
   }

   void MainWindow::slotB1()
   {
       xmotor("HTC Thunderbolt", "", lineEditUrl->text(), 480, 800, "");
   }

   void MainWindow::slotB2()
   {
       xmotor("Xperia PLAY", "", lineEditUrl->text(), 480, 854, "");
   }

   void MainWindow::slotB3()
   {
       xmotor("Droid X", "", lineEditUrl->text(), 480, 854, "");
   }

   void MainWindow::slotB4()
   {
       xmotor("Droid Pro", "", lineEditUrl->text(), 320, 480, "");
   }

   void MainWindow::slotB5()
   {
       xmotor("BlackBerry Torch 9800", "", lineEditUrl->text(), 360, 480, "");
   }
   */
