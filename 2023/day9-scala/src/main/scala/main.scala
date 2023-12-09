import scala.annotation.tailrec
import scala.io.Source
import scala.util.Using

@main
def main(): Unit = {
  val sequences = Using(Source.fromFile("day9.txt"))(source => {
    (for {
      line <- source.getLines()
    } yield line.split(' ').map(s => s.toInt).toVector).toVector
  }).get
  val results = sequences.map(s => prediction(s))
  println(s"${results.sum}")
  val results2 = sequences.map(s => prediction2(s))
  println(s"${results2.sum}")
}

def prediction(sequence: Seq[Int]): Int = {
  @tailrec
  def loop(s: Seq[Int], acc: Seq[Int]): Seq[Int] = {
    if (s.forall(n => n == 0)) {
      acc :+ 0
    } else {
      val next = s.sliding(2).map(w => w(1) - w.head).toVector
      loop(next, acc :+ s.last)
    }
  }

  val results = loop(sequence, Vector.empty)
  results.sum
}

def prediction2(sequence: Seq[Int]): Int = {
  @tailrec
  def loop(s: Seq[Int], acc: Seq[Int]): Seq[Int] = {
    if (s.forall(n => n == 0)) {
      acc :+ 0
    } else {
      val next = s.sliding(2).map(w => w(1) - w.head).toVector
      loop(next, acc :+ s.head)
    }
  }

  val results = loop(sequence, Vector.empty)
  results.reverse.reduce((a,b) => b - a)
}
