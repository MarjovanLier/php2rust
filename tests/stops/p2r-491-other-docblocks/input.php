<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

// Mago's findings that P2R-490 does not drop still stop (P2R-491): a cast to a class that
// does not exist inside a configuration property, the same docblock finding on a model
// property that P2R-401 does not read, and on a class that is no model.
final class Code extends Model
{
    /**
     * @var array<string, string>
     */
    protected $casts = ['code' => Missing::class];

    /**
     * @var list<string>
     */
    protected $appends = ['label'];
}

class Base
{
    /**
     * @var array<int, string>
     */
    protected $items = [];
}

final class Child extends Base
{
    /**
     * @var list<string>
     */
    protected $items = ['a'];
}
