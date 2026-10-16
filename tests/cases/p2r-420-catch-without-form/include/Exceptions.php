<?php

// A stand-in for two of Laravel 12.35.1's exception classes: their names and parents. It is
// not the framework.

namespace Illuminate\Database;

class RecordsNotFoundException extends \RuntimeException
{
}

namespace Illuminate\Database\Eloquent;

use Illuminate\Database\RecordsNotFoundException;

class ModelNotFoundException extends RecordsNotFoundException
{
}
