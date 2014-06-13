#include "bloc.hh"
#include "output.hh"

Bloc::Bloc(std::string environment) {
    this->environment_ = environment;
    this->filename_ = "/tmp/beer.cc"; // We need to write a temp file here!
}

Bloc::~Bloc() {
}

void Bloc::feed(const std::string &code) {
    this->code_ += code + "\n";
}

std::string Bloc::extract(void) {
    std::ofstream beer; // And we should open it here!
    beer.open(this->filename_);
    beer << this->code_ << std::endl;
    beer.close(); // Job is done

    std::string cmd;
    std::size_t pos = this->environment_.find("$^");
    if (pos != std::string::npos) {
        cmd = this->environment_;
        do {
            cmd = cmd.substr(0, pos) + this->filename_ + cmd.substr(pos + 2);
            pos = cmd.find("$^");
        } while (pos != std::string::npos);
    }
    else
        cmd = this->environment_ + " " + this->filename_;

    FILE* pipe = popen(cmd.c_str(), "r");
    if (!pipe)
        exit(error("bad environment: " + cmd));
    //else
    //    note(cmd);
    char buffer[128];
    std::string result;
    while (!feof(pipe)) {
        if (fgets(buffer, 128, pipe) != NULL)
            result += buffer;
    }
    pclose(pipe);

    return result;
}
