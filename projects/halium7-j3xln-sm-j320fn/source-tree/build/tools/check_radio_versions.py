#!/usr/bin/env python3
#
# Copyright (C) 2012 The Android Open Source Project
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

import os
import sys

try:
    from hashlib import sha1
except ImportError:
    from sha import sha as sha1

if len(sys.argv) < 2:
    sys.exit(0)

build_info = {}
with open(sys.argv[1]) as f:
    for line in f:
        line = line.strip()
        if line.startswith("require"):
            key, value = line.split()[1].split("=", 1)
            build_info[key] = value

bad = False

for item in sys.argv[2:]:
    key, fn = item.split(":", 1)

    values = build_info.get(key, None)
    if not values:
        continue
    values = values.split("|")

    with open(fn, "rb") as f:
        digest = sha1(f.read()).hexdigest()

    versions = {}
    try:
        f = open(fn + ".sha1")
    except IOError:
        if not bad:
            print()
        print("*** Error opening \"%s.sha1\"; can't verify %s" % (fn, key))
        bad = True
        continue

    with f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            h, v = line.split()
            versions[h] = v

    if digest not in versions:
        if not bad:
            print()
        print("*** SHA-1 hash of \"%s\" doesn't appear in \"%s.sha1\"" % (fn, fn))
        bad = True
        continue

    if versions[digest] not in values:
        if not bad:
            print()
        print("*** \"%s\" is version %s; not any %s allowed by \"%s\"." % (
            fn, versions[digest], key, sys.argv[1]))
        bad = True

if bad:
    print()
    sys.exit(1)