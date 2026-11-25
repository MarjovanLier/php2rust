<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

// A model property is left out of the output only in the form P2R-401 reads: protected
// $table, $casts or a literal one, or public $timestamps, each alone, with no attribute and
// no type. PHP accepts each of these, and each stops.

// An attribute on $table.
final class TableAttribute extends Model
{
    #[Marker]
    protected $table = 'codes';

    public $timestamps = false;
}

// An attribute on $timestamps.
final class TimestampsAttribute extends Model
{
    protected $table = 'codes';

    #[Marker]
    public $timestamps = false;
}

// An attribute on $casts.
final class CastsAttribute extends Model
{
    protected $table = 'codes';

    public $timestamps = false;

    #[Marker]
    protected $casts = ['code' => 'string'];
}

// An attribute on a literal property.
final class FillableAttribute extends Model
{
    protected $table = 'codes';

    public $timestamps = false;

    #[Marker]
    protected $fillable = ['code'];
}

// $table with another visibility.
final class PublicTable extends Model
{
    public $table = 'codes';

    public $timestamps = false;
}

// Two properties in one declaration.
final class TwoItems extends Model
{
    protected $table = 'codes', $fillable = ['code'];

    public $timestamps = false;
}
