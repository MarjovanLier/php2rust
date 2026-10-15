<?php

// A stand-in for an include of Data.php alone: the trait it uses is in no included file.

namespace Spatie\LaravelData;

abstract class Data
{
    use Concerns\BaseData;
}
