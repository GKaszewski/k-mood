import { CheckCircle2, AlertCircle } from "lucide-react"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Link } from "@tanstack/react-router"
import type { ImportResultResponse } from "@/api/schema"

interface WizardResultProps {
  result: ImportResultResponse
}

export function WizardResult({ result }: WizardResultProps) {
  return (
    <div className="flex flex-col gap-4">
      <Card>
        <CardHeader>
          <CardTitle className="text-sm">Import complete</CardTitle>
        </CardHeader>
        <CardContent className="flex flex-col gap-3">
          <div className="flex items-center gap-2">
            <CheckCircle2 className="h-4 w-4 text-green-500" />
            <span className="text-sm">{result.imported} entries imported</span>
          </div>

          {result.skipped > 0 && (
            <div className="flex items-center gap-2">
              <AlertCircle className="h-4 w-4 text-yellow-500" />
              <span className="text-sm">{result.skipped} entries skipped</span>
            </div>
          )}

          {result.errors.length > 0 && (
            <div className="flex flex-col gap-1">
              <span className="text-sm font-medium">Errors:</span>
              <div className="flex flex-wrap gap-1">
                {result.errors.slice(0, 5).map((err, i) => (
                  <Badge key={i} variant="destructive" className="text-xs">
                    {err}
                  </Badge>
                ))}
                {result.errors.length > 5 && (
                  <Badge variant="outline" className="text-xs">
                    +{result.errors.length - 5} more
                  </Badge>
                )}
              </div>
            </div>
          )}
        </CardContent>
      </Card>

      <Link to="/">
        <Button className="w-full">Go to dashboard</Button>
      </Link>
    </div>
  )
}
