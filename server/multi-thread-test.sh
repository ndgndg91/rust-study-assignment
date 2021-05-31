time=0;
while true;
do
  curl http://127.0.0.1:8080/sex
  if [ $time -eq 10000 ];
  then
      break;
  fi
  time=$((time+1))
done
echo "Done!"