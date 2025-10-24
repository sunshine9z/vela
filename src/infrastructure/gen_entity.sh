#!/bin/bash
sea-orm-cli generate entity -u mysql://root:12345678@localhost:3306/vela -o src/persistence/entities