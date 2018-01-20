// ---- XMotor Function ---- //

/*
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
*/
